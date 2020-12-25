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
                //format! is similar to print!, but returns a heap 
                // allocated string instead of printing to the console
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
```rust
// Globals are declared outside all other scopes.
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // Access constant in some function
    n > THRESHOLD
}

fn main() {
    let n = 16;
    // Access constant in the main thread
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });
    //=> 16 is big
}
```

#### 2.5. Types
**a) Casting**
```rust
// Suppress all warnings from casts which overflow.
#![allow(overflowing_literals)]

fn main() {
    let decimal = 65.4321_f32;
    // Error! No implicit conversion
    // let integer: u8 = decimal;

    // Explicit conversion
    let integer = decimal as u8;
    let character = integer as char;

    // Error! There are limitations in conversion rules.
    // A float cannot be directly converted to a char.
    // let character = decimal as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);
    //=> Casting: 65.4321 -> 65 -> A

    // when casting any value to an unsigned type, T,
    // T::MAX + 1 is added or subtracted until the value
    // fits into the new type

    // 1000 already fits in a u16 -> 1000
    println!("1000 as a u16 is: {}", 1000 as u16);

    // 1000 - 256 - 256 - 256 = 232
    // Under the hood, the first 8 least significant bits (LSB) are kept,
    // while the rest towards the most significant bit (MSB) get truncated.
    println!("1000 as a u8 is : {}", 1000 as u8);
    // -1 + 256 = 255
    println!("  -1 as a u8 is : {}", (-1i8) as u8);

    // For positive numbers, this is the same as the modulus -> 232
    println!("1000 mod 256 is : {}", 1000 % 256);

    // When casting to a signed type, the (bitwise) result is the same as
    // first casting to the corresponding unsigned type. If the most significant
    // bit of that value is 1, then the value is negative.

    // Unless it already fits, of course. -> 128
    println!(" 128 as a i16 is: {}", 128 as i16);
    // 128 as u8 -> -128, whose two's complement in eight bits is:
    println!(" 128 as a i8 is : {}", 128 as i8);

    // repeating the example above
    // 1000 as u8 -> 232
    println!("1000 as a u8 is : {}", 1000 as u8);
    // and the two's complement of 232 is: -24
    println!(" 232 as a i8 is : {}", 232 as i8);
}
```

**b) Numeric literals**
```rust
fn main() {
    // Suffixed literals, their types are known at initialization
    let c = 'x';
    let s = "s";
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // Unsuffixed literals, their types depend on how they are used
    let i = 1;
    let f = 1.0;

    // `size_of_val` returns the size of a variable in bytes
    println!("size of `c` in bytes: {}", std::mem::size_of_val(&c));
    println!("size of `s` in bytes: {}", std::mem::size_of_val(&s));
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
    // size of `c` in bytes: 4 //char
    // size of `s` in bytes: 16 //String
    // size of `x` in bytes: 1
    // size of `y` in bytes: 4
    // size of `z` in bytes: 4
    // size of `i` in bytes: 4
    // size of `f` in bytes: 8
}
```

**c) Inference**
```rust
fn main() {
    // Because of the annotation, the compiler knows that `elem` has type u8.
    let elem = 5u8;

    // Create an empty vector (a growable array).
    let mut vec = Vec::new();
    // At this point the compiler doesn't know the exact type of `vec`, it
    // just knows that it's a vector of something (`Vec<_>`).

    // Insert `elem` in the vector.
    vec.push(elem);
    // Aha! Now the compiler knows that `vec` is a vector of `u8`s (`Vec<u8>`)
    // TODO ^ Try commenting out the `vec.push(elem)` line
    //=> error: cannot infer type for type parameter `T`

    println!("{:?}", vec);
}
```

**d) Aliasing**
```rust
// `NanoSecond` is a new name for `u64`.
type NanoSecond = u64;
type Inch = u64;

// Use an attribute to silence warning.
#[allow(non_camel_case_types)]
type u64_t = u64;

fn main() {
    // `NanoSecond` = `Inch` = `u64_t` = `u64`.
    let nanoseconds: NanoSecond = 5 as u64_t;
    let inches: Inch = 2 as u64_t;

    // Note that type aliases *don't* provide any extra type safety, because
    // aliases are *not* new types
    println!("{} nanoseconds + {} inches = {} unit?",
             nanoseconds,
             inches,
             nanoseconds + inches);
    //=> 5 nanoseconds + 2 inches = 7 unit?
}
```

#### 2.6. Conversion
**a) From & Into**
```rust
use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        println!("In fn From Number");
        Number {value: item}
    }
}

fn main() {
    let num = Number::from(30);
    println!("My number is {:?}", num);
    //=> My number is Number { value: 30 }

    let int  = 5;
    let num2: Number = int.into(); // call Number::from(int);
    println!("My number is {:?}", num2);
    //=> My number is Number { value: 5 }
}
```

**b) TryFrom & TryInto**
```rust
use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
struct EvenNumber (i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        println!("In fn TryFrom EvenNumber");
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

fn main() {
    // TryFrom
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));
    // TryInto
    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
}
```

**c) To and from Strings**
```rust
use std::fmt;

struct Circle {
    radius: i32,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        println!("In fn Display Circle");
        write!(f, "Circle of radius {}", self.radius)
    }
}

fn main() {
    let circle =Circle {radius: 6};
    println!("{}", circle.to_string());
    //=> Circle of radius 6

    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();
    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);
    //=> Sum: 15
}
```

### 3. Expressions
```rust
fn main() {
    let x = 2u32;
    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;
        // This expression will be assigned to `y` => no semicolon
        x_cube + x_squared + x
    };
    let z = {
        // The semicolon suppresses this expression and `()` is assigned to `z`
        2 * x;
    };
    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
    // x is 2
    // y is 14
    // z is ()
}
```

### 4. Flow of Control
#### 4.1. If Else
```rust
fn main() {
    let n = 5;
    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }
    let big_n =
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");
            // This expression returns an `i32`.
            10 * n
        } else {
            println!(", and is a big number, halve the number");
            // This expression must return an `i32` as well.
            n / 2
        };
    //   ^ Don't forget to put a semicolon here! All `let` bindings need it.
    println!("{} -> {}", n, big_n);
    //=> 5 is positive, and is a small number, increase ten-fold
    //=> 5 -> 50
}
```

#### 4.2. Loop
```rust
fn main() {
    let mut count = 0u32;
    println!("Let's count until infinity!");
    // Infinite loop
    loop {
        count += 1;
        if count == 3 {
            println!("three");
            // Skip the rest of this iteration
            continue;
        }
        println!("{}", count);
        if count == 5 {
            println!("OK, that's enough");
            // Exit this loop
            break;
        }
    }
    // Let's count until infinity!
    // 1
    // 2
    // three
    // 4
    // 5
    // OK, that's enough
}
```

**a) Nesting and labels**
```rust
#![allow(unreachable_code)]
fn main() {
    'outer: loop {
        println!("Entered the outer loop");
        'inner: loop {
            println!("Entered the inner loop");
            // This would break only the inner loop
            // break;

            // This breaks the outer loop
            break 'outer;
        }
        println!("This point will never be reached");
    }
    println!("Exited the outer loop");
    // Entered the outer loop
    // Entered the inner loop
    // Exited the outer loop
}
```

**b) Returning from loops**
```rust
fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    assert_eq!(result, 20);
}
```

#### 4.3. while
```rust
fn main() {
    // A counter variable
    let mut n = 1;
    // Loop while n is less than 101
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
        // Increment counter
        n += 1;
    }
}
```

#### 4.4. for loops
```rust

```
