# diffrs
A diff tool built using longest common subsequence algorithm.

## Usage

### Files
```bash
diffrs <file_1> <file_2>
```

### Strings
Use -w flag to compare strings.

```bash
diffrs -w <string_1> <string_2>
```

## Build
Install [Rust](https://www.rust-lang.org/tools/install) and build using Cargo.

```bash
git clone https://github.com/dhirajgagrai/diffrs.git
cd diffrs 

# Compile
cargo build --release

# Run
cargo run --release -- examples/kitten.txt examples/sitting.txt
```

To use `diffrs` as command, create a symlink or copy the executable to $PATH.
```bash
sudo cp target/release/diffrs /usr/local/bin
```

## Project goals
- [ ] Add line-oriented format
- [ ] Support hunks
- [ ] String comparisons from CLI
