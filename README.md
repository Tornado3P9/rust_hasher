# rust-hasher

```
Usage: rust-hasher [OPTIONS]

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
```

```bash
# Create a checksum for all files in the directory, including subdirectories (recursively):
rust-hasher -r | tee checksums.txt
rust-hasher -r > checksums.txt

# If the file checksums.txt contains a line with an old checksum of itself
grep "checksums.txt" checksums.txt
# and it bothers you, just delete it with
sed -i '/checksums.txt/d' checksums.txt
```

```bash
# Verify checksums list in a file and filter only for "not ok" results
rust-hasher -c checksums.txt | grep -v OK
```

```bash
# Append a new file to the list:
rust-hasher -f ./new_file | tee -a checksums.txt
rust-hasher -f ./new_file >> checksums.txt
```
