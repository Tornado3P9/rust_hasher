# rust-hasher

Syntax:
```bash
# Print syntax when there are no additional arguments
rust-hasher [] [-d|--directory] [-r|--recursive] [-f|--file <file_path>] [-c|--check <checksum_file>]

# -d: Call the hash program on all files in current directory
# -r: In a structured directory also include the files in subdirectories
# -f: Call the hash program on a single file
# -c: Verify previously generated checksums.txt file
# -V: Print version number
```

Create a checksum for all files in the directory, including subdirectories (recursively):
```bash
rust-hasher -r | tee checksums.txt
rust-hasher -r > checksums.txt

# If the file checksums.txt contains a line with an old checksum of itself
grep "checksums.txt" checksums.txt
# and it bothers you, just delete it with
sed -i '/checksums.txt/d' checksums.txt
```

Verify checksums list in a file and filter only for "not ok" results
```bash
rust-hasher -c checksums.txt | grep -v OK
```

Append a new file to the list:
```bash
rust-hasher -f ./new_file | tee -a checksums.txt
rust-hasher -f ./new_file >> checksums.txt
```
