# diffrs
A diff tool built using longest common subsequence algorithm.

## Usage
```bash
diffrs <file_1> <file_2>
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

Create a symlink or copy the executable to a directory available in $PATH to use `diffrs` systemwide.
```bash
sudo cp target/release/diffrs /usr/local/bin
```
