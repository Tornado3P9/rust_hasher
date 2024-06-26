use crc32fast::Hasher;
use std::env;
use std::fs::File;
// use std::io::{self, BufRead, BufReader, Read, Write};
use std::io::{self, BufRead, BufReader, Read};
use std::path::Path;
use walkdir::WalkDir;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        //  when there are no additional arguments
        1 => Ok(println!("Usage: rust_hasher [] [-d|--directory] [-r|--recursive] [-f|--file <file_path>] [-c|--check <checksum_file>]")),
        // Call the function on all files in current directory
        2 if args[1] == "-d" || args[1] == "--directory" => calculate_checksums_in_current_dir(),
        // In a structured directory also include the files in subdirectories
        2 if args[1] == "-r" || args[1] == "--recursive" => {
            calculate_checksums_in_current_structured_directory()
        }
        // Call the function on a single file
        3 if args[1] == "-f" || args[1] == "--file" => {
            calculate_checksum_for_single_file(args[2].clone().to_string())
        }
        // Verify previously generated checksums.txt file
        3 if args[1] == "-c" || args[1] == "--check" => {
            verify_checksums_from_list(args[2].clone().to_string())
        }
        _ => {
            eprintln!("Usage: rust_hasher [] [-d|--directory] [-r|--recursive] [-f|--file <file_path>] [-c|--check <checksum_file>]");
            std::process::exit(1);
        }
    }
}

fn calculate_checksums_in_current_dir() -> io::Result<()> {
    for entry in WalkDir::new(".")
        .min_depth(1) // Start at the current directory level
        .max_depth(1) // Do not descend into subdirectories
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    // Filter out directories
    {
        let path = entry.path();
        let checksum = calculate_crc32(&path)?;
        println!("{:08x} {}", checksum, path.display());
    }
    Ok(())
}

fn calculate_checksums_in_current_structured_directory() -> io::Result<()> {
    // let output_file = File::create("checksums.txt")?;
    // let mut writer = io::BufWriter::new(output_file);

    for entry in WalkDir::new(".")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        let checksum = calculate_crc32(&path)?;
        println!("{:08x} {}", checksum, path.display());
        // writeln!(writer, "{:08x} {}", checksum, path.display())?;
    }

    Ok(())
}

fn calculate_checksum_for_single_file(file_path: String) -> io::Result<()> {
    let checksum = calculate_crc32(&file_path)?;
    println!("{:08x} {}", checksum, file_path);
    Ok(())
}

fn calculate_crc32<P: AsRef<Path>>(path: P) -> io::Result<u32> {
    // Open the file
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    // Create a new CRC32 hasher
    let mut hasher = Hasher::new();

    // Buffer to hold file chunks
    // let mut buffer = vec![0; 8 * 1024 * 1024]; // 8MB buffer
    let mut buffer = [0; 1024 * 1024]; // 1MB buffer

    // Read the file in chunks and update the hasher
    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    // Finalize the hash
    Ok(hasher.finalize())
}

fn verify_checksums_from_list(checksum_file_path: String) -> io::Result<()> {
    // println!("checksum_file_path: {}", checksum_file_path.clone());
    let file = File::open(checksum_file_path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;

        let mut parts = line.splitn(2, ' ');
        let checksum_str = parts.next().unwrap_or("");
        let filename = parts.next().unwrap_or("");

        match verify_file_integrity(filename, checksum_str) {
            Ok(valid) => {
                if valid {
                    println!("{}: OK", filename);
                } else {
                    println!("{}: FAILED", filename);
                }
            }
            Err(e) => {
                eprintln!("Error verifying {}: {}", filename, e);
            }
        }
    }

    Ok(())
}

fn verify_file_integrity(filename: &str, expected_checksum: &str) -> io::Result<bool> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);
    let mut hasher = Hasher::new();
    // let mut buffer = vec![0; 8 * 1024 * 1024]; // 8MB buffer
    let mut buffer = [0; 1024 * 1024]; // 1MB buffer

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }

    let checksum = hasher.finalize();
    let calculated_checksum_str = format!("{:08x}", checksum);
    Ok(calculated_checksum_str == expected_checksum)
}
