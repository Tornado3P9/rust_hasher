# rust_hasher

Syntax:
```bash
rust_hasher [] [-d|--directory] [-r|--recursive] [-f|--file <file_path>] [-c|--check <checksum_file>]
```

Create a checksum for all files in the directory, including subdirectories (recursively):
```bash
rust_hasher -r | tee checksums.txt
rust_hasher -r > checksums.txt
```

Verify checksums list in a file and filter only for "not ok" results
```bash
rust_hasher -c checksums.txt | grep -v OK
```

Append a new file to the list:
```bash
rust_hasher -f ./new_file | tee -a checksums.txt
rust_hasher -f ./new_file >> checksums.txt
```
