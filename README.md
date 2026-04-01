# awesum

## Installation

1. Install Rust from http://rustup.rs/
2. Run `cargo install --git https://github.com/j10s/awesum.git`

## Usage
```
Print or check checksums

Usage: awesum.exe [OPTIONS] --algorithm <ALGORITHM> [FILE]...

Arguments:
  [FILE]...  One or more file paths. When check is set, only one file path is supported

Options:
  -a, --algorithm <ALGORITHM>  Hashing algorithm to use [possible values: crc32, md5, sha1, sha2224, sha2256, sha2384, sha2512, sha3224, sha3256, sha3384, sha3512, blake2b, blake2s, blake3]
  -j, --jobs <JOBS>            Maximum number of files to hash at once
  -c, --check                  Check sums against given list
  -i, --ignore-missing         Don't fail or report status for missing files
  -q, --quiet                  Don't show progress
  -s, --status                 Don't output anything, status code shows success
  -w, --warn                   Warn about improperly formatted checksum lines
  -h, --help                   Print help
  -V, --version                Print version
```

### Examples
#### Print
`awesum -a crc32 file1.bin file2.bin file3.bin`

You probably want to redirect this output to a file e.g

`awesum -a crc32 file1.bin file2.bin file3.bin > sums.sfv`


#### Check
`awesum -c -a crc32 sums.sfv`
