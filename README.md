# rust_tour
rust_tour is a cheat sheet, quick reference to learn rust programming  

## Install Rust
```bash
# https://www.rust-lang.org/tools/install
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

#==> reboot computer

# check
rustc --version
cargo --version
```

## Init project
```bash
# New project rust_tour
cargo new rust_tour
cd rust_tour

# Build project
cargo build

# Building for Release
cargo build --release

# Run project
cargo run

# Check errors
cargo check

# Clean project
cargo clean

# Run test
cargo test
```

## Rust IDE Tool
IntelliJ Idea + plugin rust  
Android Studio + plugin rust  
VSCode + plugin rust  

## Getting Started
Reference code detail in file [main.rs](https://github.com/congnghia0609/rust_tour/blob/main/src/main.rs)  

```rust
// Hello world
fn main() {
    println!("Hello, world!");
}
```