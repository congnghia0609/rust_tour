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
fn main() {
    println!("Hello, world!"); // Hello, world!
    println!("{} days", 31); // 31 days
    /* Print correct number of arguments */
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    // Alice, this is Bob. Bob, this is Alice

    /* Print format binary */
    println!("format binary of {} is {:b}", 3, 3);
    // format binary of 3 is 11

    /* Print white spaces */
    println!("|{number:>width$}|", number=1, width=6);
    // |     1|

    /* Print Structure */
    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8
    }
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
    println!("{:?}", peter);
    // Person { name: "Peter", age: 27 }

    /* Pretty print */
    println!("{:#?}", peter);
    /*
    Person {
        name: "Peter",
        age: 27,
    }
    */
}
```

### 2. Data type
#### 2.1. Primitives
- signed integers: i8, i16, i32, i64, i128 and isize (pointer size)
- unsigned integers: u8, u16, u32, u64, u128 and usize (pointer size)
- floating point: f32, f64
- char Unicode scalar values like 'a', 'α' and '∞' (4 bytes each)
- bool either true or false
- and the unit type (), whose only possible value is an empty tuple: ()
- arrays like [1, 2, 3]
- tuples like (1, true)

```rust
let logical: bool = true;
let a_float: f64 = 1.0;  // Regular annotation
let an_integer   = 5i32; // Suffix annotation
let default_float   = 3.0; // `f64`
let default_integer = 7;   // `i32`
// A type can also be inferred from context 
let mut inferred_type = 12; // Type i64 is inferred from another line
inferred_type = 4294967296i64;
// A mutable variable's value can be changed.
let mut mutable = 12; // Mutable `i32`
mutable = 21;
// Variables can be overwritten with shadowing.
let mutable = true;
```

#### 2.2. Tuples
A tuple is a collection of values of different types.  

```rust
// A tuple with a bunch of different types
let long_tuple = (1u8, 2u16, 3u32, 4u64,
                  -1i8, -2i16, -3i32, -4i64,
                  0.1f32, 0.2f64,
                  'a', true);
// Values can be extracted from the tuple using tuple indexing
println!("long tuple first value: {}", long_tuple.0); // long tuple first value: 1
println!("long tuple second value: {}", long_tuple.1); // long tuple second value: 2

// Tuples can be tuple members
let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
// Tuples are printable
println!("tuple of tuples: {:?}", tuple_of_tuples);
// tuple of tuples: ((1, 2, 2), (4, -1), -2)

//tuples can be destructured to create bindings
let tuple = (1, "hello", 4.5, true);
let (a, b, c, d) = tuple;
println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);
// 1, "hello", 4.5, true
```

#### 2.3. Arrays
An array is a collection of objects of the same type T, stored in contiguous memory.  
```rust
// Fixed-size array (type signature is superfluous)
let xs: [i32; 5] = [1, 2, 3, 4, 5];
println!("xs: {:?}", xs); // xs: [1, 2, 3, 4, 5]
println!("xs.len: {:?}", xs.len()); // xs.len: 5
```



