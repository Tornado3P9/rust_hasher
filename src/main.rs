use crc32fast::Hasher;
use rayon::prelude::*; // .par_bridge(): Use parallel iterator
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::path::Path;
use walkdir::WalkDir;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 if args[1] == "-V" || args[1] == "--version" => Ok(println!("{} v0.1.2", args[0])),
        2 if args[1] == "-d" || args[1] == "--directory" => calculate_checksums_in_current_dir(false),
        3 if (args[1] == "-d" || args[1] == "--directory") && args[2] == "local" => calculate_checksums_in_current_dir(true),
        2 if args[1] == "-r" || args[1] == "--recursive" => calculate_checksums_in_current_structured_directory(false),
        3 if (args[1] == "-r" || args[1] == "--recursive") && args[2] == "local" => calculate_checksums_in_current_structured_directory(true),
        3 if args[1] == "-f" || args[1] == "--file" => calculate_checksum_for_single_file(args[2].clone()),
        3 if args[1] == "-c" || args[1] == "--check" => verify_checksums_from_list(args[2].clone()),
        _ => Ok(print_usage()),
    }
}

fn print_usage() {
    println!(
"Usage: rust-hasher [OPTIONS]

Options:
  -d, --directory          Calculate checksums in the current directory.
  -d, --directory local    Calculate checksums in the current directory (local mode).
  -r, --recursive          Calculate checksums recursively in the current directory.
  -r, --recursive local    Calculate checksums recursively in the current directory (local mode).
  -f, --file <file_path>   Calculate checksum for a single file specified by <file_path>.
  -c, --check <checksum_file> Verify checksums from a specified <checksum_file>.
  -V, --version            Display the version information.

Examples:
  rust-hasher -d
  rust-hasher --file ./example.txt
  rust-hasher -c checksums.txt
");
}

fn calculate_checksums_in_current_dir(local: bool) -> io::Result<()> {
    WalkDir::new(".")
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .par_bridge()
        .for_each(|entry| {
            let path = entry.path();
            let display_path = if local { path.display().to_string() } else { path.canonicalize().unwrap().display().to_string() };
            match calculate_crc32(path) {
                Ok(checksum) => println!("{:08x} {}", checksum, display_path),
                Err(e) => eprintln!("Error processing {}: {}", display_path, e),
            }
        });
    Ok(())
}

fn calculate_checksums_in_current_structured_directory(local: bool) -> io::Result<()> {
    WalkDir::new(".")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .par_bridge()
        .for_each(|entry| {
            let path = entry.path();
            let display_path = if local { path.display().to_string() } else { path.canonicalize().unwrap().display().to_string() };
            match calculate_crc32(path) {
                Ok(checksum) => println!("{:08x} {}", checksum, display_path),
                Err(e) => eprintln!("Error processing {}: {}", display_path, e),
            }
        });
    Ok(())
}

fn calculate_checksum_for_single_file(file_path: String) -> io::Result<()> {
    let checksum: u32 = calculate_crc32(&file_path)?;
    println!("{:08x} {}", checksum, file_path);
    Ok(())
}

fn calculate_crc32<P: AsRef<Path>>(path: P) -> io::Result<u32> {
    let file: File = File::open(path)?;
    let mut reader: BufReader<File> = BufReader::new(file);
    let mut hasher: Hasher = Hasher::new();
    let mut buffer: [u8; 1048576] = [0; 1024 * 1024]; // 1MB buffer

    loop {
        let bytes_read: usize = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    Ok(hasher.finalize())
}

fn verify_checksums_from_list(checksum_file_path: String) -> io::Result<()> {
    let file: File = File::open(checksum_file_path)?;
    let reader: BufReader<File> = BufReader::new(file);

    reader.lines().par_bridge().for_each(|line| {
        if let Ok(line) = line {
            let mut parts: std::str::SplitN<'_, char> = line.splitn(2, ' ');
            let checksum_str: &str = parts.next().unwrap_or("");
            let filename: &str = parts.next().unwrap_or("");

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
    });

    Ok(())
}

fn verify_file_integrity(filename: &str, expected_checksum: &str) -> io::Result<bool> {
    let file: File = File::open(filename)?;
    let mut reader: BufReader<File> = BufReader::new(file);
    let mut hasher: Hasher = Hasher::new();
    let mut buffer: [u8; 1048576] = [0; 1024 * 1024]; // 1MB buffer

    loop {
        let count: usize = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }

    let checksum: u32 = hasher.finalize();
    let calculated_checksum_str: String = format!("{:08x}", checksum);
    Ok(calculated_checksum_str == expected_checksum)
}
