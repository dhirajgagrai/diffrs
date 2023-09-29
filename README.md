# diffr
A diff tool built with Rust using longest common subsequence algorithm.

## Usage
```bash
diffr <file_1> <file_2>
```

## Build
Install [Rust](https://www.rust-lang.org/tools/install) and build using Cargo.
```bash
git clone https://github.com/dhirajgagrai/diffr.git
cd diffr 

# Compile
cargo build --release

# Run
cargo run --release -- examples/kitten.txt examples/sitting.txt
```

Create a symlink or copy the executable to a directory available in $PATH to use `diffr` systemwide.
```bash
sudo cp target/release/diffr /usr/local/bin
```
