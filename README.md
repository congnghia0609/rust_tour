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
println!("first element: {}", xs[0]); // first element: 1
println!("xs[0..4]: {:?}", &xs[0..4]); // xs[0..4]: [1, 2, 3, 4]
println!("xs[0..=4]: {:?}", &xs[0..=4]); // xs[0..=4]: [1, 2, 3, 4, 5]
```

#### 2.4. Custom Types
**a) Structures**
```rust
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person{name, age};
    // Print struct
    println!("{:?}", peter);
    //=> Person { name: "Peter", age: 27 }

    // Instantiate a Point
    let point: Point = Point{x: 10.3, y: 0.4};
    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);
    //=> point coordinates: (10.3, 0.4)

    // Make a new point by using struct update syntax to use the fields of our other one
    let bottom_right = Point{x: 5.2, ..point};
    // bottom_right.y will be the same as point.y because we used that field from point
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);
    //=> second point: (5.2, 0.4)

    // Destructure the point using a let binding
    let Point{x: top_edge, y: left_edge} = point;
    let _resctangle = Rectangle{
        // struct instantiation is an expression too
        top_left: Point{x: left_edge, y: top_edge},
        bottom_right: bottom_right,
    };
    // Print struct
    println!("{:?}", _resctangle);
    //=> Rectangle { top_left: Point { x: 0.4, y: 10.3 }, bottom_right: Point { x: 5.2, y: 0.4 } }

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);
    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);
    //=> pair contains 1 and 0.1

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);
    //=> pair contains 1 and 0.1
}
```

**b) Enums**
```rust
use crate::List::*;

// Create an enum keyword allows the creation of a type which may be of a few diffent.
// Any variant is valid ad a struct is also valid as an enum.
enum WebEvent {
    // An enum may either be unit-like
    PageLoad,
    PageUnload,
    // like tuple structs,
    KeyPress(char),
    Paste(String),
    // or c-like structures.
    Click {x: i64, y: i64},
}

// A function which takes a WebEvent enum as an argument and returns nothing.
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Destructure c from inside the enum.
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // Destructure Click into x and y.
        WebEvent::Click {x, y} => {
            println!("clicked at x={}, y={}", x, y);
        },
    }
}

// Type aliases
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}
impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}
// Creates a type alias
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

// A common use for enums is to create a linked-list:
enum List {
    // Cons: Tuple struct that wraps an element and a pointer to the next node.
    Cons(u32, Box<List>),
    // Nil: A node that signifies the end of the linked list.
    Nil,
}
// Methods can be attached to an enum
impl List {
    // Create an empty list
    fn new() -> List {
        // Nil has tyle List
        Nil
    }

    // Consume a list, and return the same list with a new element at its front
    fn prepend(self, elem: u32) -> List {
        // Cons also has type List
        Cons(elem, Box::new(self))
    }

    // Return the length of the list
    fn len(&self) -> u32 {
        // self has tobe matched, because the behavior of this method
        // depends on the variant of self.
        // self has type &List, and *self has type List, matching on
        // concrete type T is preferred over a match on a reference &T
        match *self {
            // Can't take ownership of the tail, because self is borrowed;
            // instead take a reference to the tail
            Cons(_, ref tail) => 1 + tail.len(),
            // Base Case: An empty list has zero length
            Nil => 0,
        }
    }

    // Return representation of the list as a (heap allocated) string
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                //format! is similar to print!, but returns a heap allocated string instead of printing to the console
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            }
        }
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    // to_owned() creates an owned String from a string slice.
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click {x: 20, y: 80};
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed); //=> pressed 'x'.
    inspect(pasted); //=> pasted "my text".
    inspect(click); //=> clicked at x=20, y=80
    inspect(load); //=> page loaded
    inspect(unload); //=> page unloaded

    // We can refer to each variant via its alias, not its long and inconvenient name.
    let opt = Operations::Add;
    println!("Operations::Add {}", opt.run(2, 3));
    //=> Operations::Add 5

    // Create an empty linked list
    let mut list = List::new();
    // Prepend some elements
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);
    // Show the final stae of the list
    println!("linked list has length: {}", list.len());
    //=> linked list has length: 3
    println!("{}", list.stringify());
    //=> 3, 2, 1, Nil
}
```

**c) constants**


