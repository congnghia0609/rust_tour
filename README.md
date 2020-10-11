# rust_tour
**rust_tour** is a cheat sheet, quick reference to learn rust programming  

## Install Rust
```shell script
# https://www.rust-lang.org/tools/install
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

#==> reboot computer

# check
rustc --version
cargo --version
```

## Init project
```shell script
# New project rust_tour
cargo new rust_tour
cd rust_tour

# New module library hello_lib
cargo new hello_lib --lib

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
- IntelliJ Idea + plugin rust  
- Android Studio + plugin rust  
- VSCode + plugin rust  

## Getting Started  

### Ownership Rules
- Each value in Rust has a variable that's called its owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

### The Rules of References
- At any given time, you can have either one mutable reference or any number of immutable references.
- References must always be valid.

### Smart Pointers
- `Box<T>` for allocating values on the heap
- `Rc<T>` a reference counting type that enables multiple ownership
- `Ref<T>` and `RefMut<T>`, accessed through `RefCell<T>`, a type that enforces the borrowing rules at runtime instead of compile time
- `Arc<T>` (atomically reference counted) is a type like `Rc<T>` that is safe to use in concurrent situations.


### 1. Hello world
```rust
// Hello world
fn main() {
    println!("Hello, world!");
}
```

