# diffrs
A diff tool that employs the longest common subsequence algorithm to compare files and strings.

## Usage

### Files
```bash
diffrs <file_1> <file_2>
```

### Strings
To compare two strings, add the -w flag:
```bash
diffrs -w <string_1> <string_2>
```

## Building and Installation
To get started first install [Rust](https://www.rust-lang.org/tools/install).
```bash
git clone https://github.com/dhirajgagrai/diffrs.git
cd diffrs 

# Compile
cargo build --release

# Run
cargo run --release -- examples/kitten.txt examples/sitting.txt
```

To use `diffrs` command, create a symlink or copy the executable to a directory in your $PATH.
```bash
sudo cp target/release/diffrs /usr/local/bin
```

## Project Goals
- [ ] Add line-oriented format
- [ ] Support hunks
- [ ] String comparisons from CLI

Feel free to contribute, report issues, or provide feedback to help improve the project.
