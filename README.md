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
fn main() {
    // a) for and range
    // n will take the values: 1, 2, ..., 100 in each iteration
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // b) for and iterators
    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    // Hello Bob
    // Hello Frank
    // There is a rustacean among us!

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    // Hello Bob
    // Hello Frank
    // There is a rustacean among us!

    let mut names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }
    println!("names: {:?}", names);
    //=> names: ["Hello", "Hello", "There is a rustacean among us!"]
}
```

#### 4.5. match
```rust
fn main() {
    let number = 13;
    println!("Tell me about {}", number);
    match number {
        // Match a single value
        1 => println!("One!"),
        // Match several values
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // Match an inclusive range
        13..=19 => println!("A teen"),
        // Handle the rest of cases
        _ => println!("Ain't special"),
    }
    // Tell me about 13
    // A teen

    let boolean = true;
    // Match is an expression too
    let binary = match boolean {
        // The arms of a match must cover all the possible values
        false => 0,
        true => 1,
    };
    println!("{} -> {}", boolean, binary);
    //=> true -> 1
}
```

**a) Destructuring**
```rust
// allow required to silence warning because only one variant is used.
#[allow(dead_code)]
enum Color {
    // These 3 are specified solely by their name.
    Red,
    Blue,
    Green,
    // These likewise tie u32 tuple to different names: color models.
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn main() {
    // 1. tuples
    let triple = (0, -2, 3);
    println!("Tell me about {:?}", triple);
    // Match can be used to destructure a tuple
    match triple {
        // Destructure the second and third elements
        (0, y, z) => println!("First is '0', 'y' is {:?}, and 'z' is {:?}", y, z),
        (1, ..) => println!("First is '1' and the rest doesn't matter"),
        // '..' can be the user ignore the rest of the tuple
        _ => println!("It doesn't matter what they are"),
        // '_' means don't bind the value to a variable
    }
    //=> Tell me about (0, -2, 3)
    //=> First is '0', 'y' is -2, and 'z' is 3


    // 2. enums
    let color = Color::RGB(122, 17, 40);
    println!("What color is it?");
    // An enum can be destructured using a match.
    match color {
        Color:: Red => println!("The color is Red!"),
        Color:: Blue => println!("The color is Blue!"),
        Color:: Green => println!("The color is Green!"),
        Color:: RGB(r, g, b) =>
            println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color:: HSV(h, s, v) =>
            println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color:: HSL(h, s, l) =>
            println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color:: CMY(c, m, y) =>
            println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color:: CMYK(c, m, y, k) =>
            println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!", c, m, y, k),
        // Don't need another arm because all variants have been examined
    }
    //=> What color is it?
    //=> Red: 122, green: 17, and blue: 40!


    // 3. pointers/ref
    //      Dereferencing uses *
    //      Destructuring uses &, ref, and ref mut
    // Assign a reference of type i32.
    // The & signifies there is a reference being assigned.
    let reference = &4;
    match reference {
        // If reference is pattern matched against &val,
        // it results in a comparison like:
        // &i32
        // &val
        // We see that if the matching & are dropped,
        // the the i32 should be assigned to val.
        &val => println!("Got a value via destructuring: {:?}", val),
    }
    //=> Got a value via destructuring: 4

    // To avoid the &, you dereference before matching.
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }
    //=> Got a value via dereferencing: 4

    // What if you don't start with a reference? reference was a &
    // because the right side was already a reference. This is not
    // a reference because the right side is not one.
    let _not_a_reference = 3;
    // Accordingly, by defining 2 values without reference, references
    // can be retrieved via ref and ref mut.
    let value = 5;
    let mut mut_value = 6;
    // Use ref keyword to create a reference.
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }
    //=> Got a reference to a value: 5
    // Use ref mut similarly.
    match mut_value {
        ref mut m => {
            // Got a reference. Gotta dereference it before we can
            // add anything to it.
            *m += 10;
            println!("We added 10. mut_value: {:?}", m);
        },
    }
    //=> We added 10. mut_value: 16


    // 4. structs
    struct Foo {
        x: (u32, u32),
        y: u32,
    }
    // Try changing the values in the struct to see what happens
    let foo = Foo { x: (1, 2), y: 3 };
    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1, b = {},  y = {} ", b, y),
        // you can destructure structs and rename the variables,
        // the order is not important
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),
        // and you can also ignore some variables:
        Foo { y, .. } => println!("y = {}, we don't care about x", y),
        // this will give an error: pattern does not mention field `x`
        //Foo { y } => println!("y = {}", y),
    }
    //=> First of x is 1, b = 2,  y = 3
}
```

**b) Guards --> filter**
```rust
fn main() {
    let pair = (2, -2);
    // TODO ^ Try different values for `pair`
    println!("Tell me about {:?}", pair);
    match pair {
        (x, y) if x == y => println!("These are twins"),
        // The ^ `if condition` part is a guard
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation..."),
    }
    //=> Tell me about (2, -2)
    //=> Antimatter, kaboom!
}
```

**c) Binding**
```rust
// A function age which returns a u32
fn age() -> u32 {
    15
}

// Destructure enum
fn some_number() -> Option<u32> {
    Some(42)
}

fn main() {
    println!("Tell me what type of person you are");
    match age() {
        0 => println!("I haven't celebrated my first birthday yet"),
        // Counld match 1..=12 directly but then what age
        // would the child be? Instead, bind to n for the
        // sequence of 1..=12. Now the age can be reported.
        n @ 1..=12 => println!("I'm a child of age {:?}", n),
        n @ 13..=19 => println!("I'm a teen of age {:?}", n),
        // Nothing bound. Return the result.
        n => println!("I'm an old person of age {:?}", n),
    }
    //=> Tell me what type of person you are
    //=> I'm a teen of age 15

    // Destructure enum
    match some_number() {
        // Got Some variant, match if its value, bound to n,
        // is equal to 42.
        Some(n @ 42) => println!("The Answer: {}!", n),
        // Match any other number.
        Some(n) => println!("Not interesting... {}", n),
        // Match anything else (None variant).
        _ => (),
    }
    //=> The Answer: 42!
}
```

#### 4.6. If let
```rust
// Our example enum
enum Foo {
    Bar,
    Baz,
    Qux(u32)
}

fn main() {
    // All have type Option<i32>
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;
    // The if let construct reads: if let destructures number into
    // Some(i), evaluate the block({}).
    if let Some(i) = number {
        println!("Matched {:?}", i);
    }
    //=> Matched 7
    // If you need to specify a failure, use an else:
    if let Some(i) = letter {
        println!("Matched {:?}", i);
    } else {
        // Destructure failed. Change to the failure case.
        println!("Didn't match a number. Let's go with a letter!");
    }
    //=> Didn't match a number. Let's go with a letter!

    // Provide an altered failing condition.
    let i_like_letters = false;
    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
        // Destructure failed. Evaluate an else if condition to see if the
        // alternate failure branch should be taken:
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        // The condition evaluated false. This branch is the default:
        println!("I don't like letters. Let's go with an emoticon :)!");
    }
    //=> I don't like letters. Let's go with an emoticon :)!

    // Create example variables
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);
    // Variable a match Foo::Bar
    if let Foo::Bar = a {
        println!("a is foobar");
    }
    //=> a is foobar
    // Variable b does not match Foo::Bar
    if let Foo::Bar = b {
        println!("b is foobar");
    }
    // Variable c matches Foo::Qux which has a value
    // Similar to Some() in the previous example
    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }
    //=> c is 100
    // Binding also works with if let
    if let Foo::Qux(value @ 100) = c {
        println!("c is one hundred");
    }
    //=> c is one hundred
}
```

#### 4.7. While let
```rust
fn main() {
    // Make optional of type Option<i32>
    let mut optional = Some(0);
    // This reads: while let destrutures optional into
    // Some(i), evaluate the block({}). Else break.
    while let Some(i) = optional {
        if i > 3 {
            println!("Greater than 3, quit!");
            optional = None;
        } else {
            println!("i is {:?}. Try again.", i);
            optional = Some(i + 1);
        }
        // ^ Less rightward drift and doesn't require
        // explicitly handling the failing case.
    }
    // i is 0. Try again.
    // i is 1. Try again.
    // i is 2. Try again.
    // i is 3. Try again.
    // Greater than 3, quit!
}
```

### 5. Functions
#### 5.1. Methods
```rust
struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all Point methods go in here
impl Point {
    // This is a static method
    // Static methods don't need to be called by an instance
    // These methods are generally used as constructors
    fn origin() -> Point {
        Point{x: 0.0, y: 0.0}
    }

    // Another static method, taking two arguments:
    fn new(x: f64, y: f64) -> Point {
        Point{x, y}
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is an instance method
    // &self is sugar for self: &Self, where Self is the type of the
    // caller object. In this case Self = Rectangle
    fn area(&self) -> f64 {
        // self gives access to the struct fields via the dot operator
        let Point{x: x1, y: y1} = self.p1;
        let Point{x: x2, y: y2} = self.p2;
        // abs is a f64 method that returns the absolute value of the caller
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point{x: x1, y: y1} = self.p1;
        let Point{x: x2, y: y2} = self.p2;
        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // &mut self desugars to self:: &mut Self
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;
        self.p1.y += y;
        self.p2.y += y;
    }
}

// Pair owns resources: two heap allocated integers
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method consumes the resources of the caller object
    // self desugars to self: Self.
    fn destroy(self) {
        // Destructure self
        let Pair(first, second) = self;
        println!("Destroying Pair({}, {})", first, second);
        // first and second go out of scope and get freed
    }
}

fn main() {
    let rectangle = Rectangle {
        // Static methods are called using double colons
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };
    // Instance methods are called using the dot operator
    // Note that the first argument &self is implicitly passed, i.e.
    // rectangle.perimeter() === Rectangle::perimeter(&rectangle)
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };
    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    // rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line
    // Okay! Mutable objects can call mutable methods
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));
    pair.destroy();
    // Error! Previous `destroy` call "consumed" `pair`
    // pair.destroy();
    // TODO ^ Try uncommenting this line
}
```

#### 5.2. Closures (lambda expressions or lambdas)
```rust
fn main() {
    // Increment via closures and functions.
    fn function(i: i32) -> i32 {i + 1}
    // Closures are anonymous, here we are binding them to references
    // Annotation is identical to function annotation but is optional
    // as are the {} wrapping the body. These nameless functions
    // are assigned to appropriately named variables.
    let closure_annotated = |i: i32| -> i32 {i + 1};
    let closure_inferred = |i| i + 1;

    let i = 1;
    // Call the function and closures.
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));
    //=> function: 2
    //=> closure_annotated: 2
    //=> closure_inferred: 2

    // A closure taking no arguments which returns an i32.
    // The return type is inferred.
    let one = || 1;
    println!("closure returning one: {}", one());
    //=> closure returning one: 1
}
```

**a) Capturing**
```rust
fn main() {
    use std::mem;

    let color = String::from("green");
    // A closure to print `color` which immediately borrows (`&`) `color` and
    // stores the borrow and closure in the `print` variable. It will remain
    // borrowed until `print` is used the last time.
    //
    // `println!` only requires arguments by immutable reference so it doesn't
    // impose anything more restrictive.
    let print = || println!("`color`: {}", color);
    // Call the closure using the borrow.
    print();
    // `color` can be borrowed immutably again, because the closure only holds
    // an immutable reference to `color`.
    let _reborrow = &color;
    print();
    // A move or reborrow is allowed after the final use of `print`
    let _color_moved = color;


    let mut count = 0;
    // A closure to increment `count` could take either `&mut count` or `count`
    // but `&mut count` is less restrictive so it takes that. Immediately
    // borrows `count`.
    //
    // A `mut` is required on `inc` because a `&mut` is stored inside. Thus,
    // calling the closure mutates the closure which requires a `mut`.
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };
    // Call the closure using a mutable borrow.
    inc();
    // The closure still mutably borrows `count` because it is called later.
    // An attempt to reborrow will lead to an error.
    // let _reborrow = &count;
    // ^ TODO: try uncommenting this line.
    inc();
    // The closure no longer needs to borrow `&mut count`. Therefore, it is
    // possible to reborrow without an error
    let _count_reborrowed = &mut count;


    // A non-copy type.
    let movable = Box::new(3);
    // `mem::drop` requires `T` so this must take by value. A copy type
    // would copy into the closure leaving the original untouched.
    // A non-copy must move and so `movable` immediately moves into
    // the closure.
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };
    // `consume` consumes the variable so this can only be called once.
    consume();
    // consume();
    // ^ TODO: Try uncommenting this line.


    // `Vec` has non-copy semantics.
    let haystack = vec![1, 2, 3];
    let contains = move |needle| haystack.contains(needle);
    println!("{}", contains(&1)); // true
    println!("{}", contains(&4)); // false
    // println!("There're {} elements in vec", haystack.len());
    // ^ Uncommenting above line will result in compile-time error
    // because borrow checker doesn't allow re-using variable after it
    // has been moved.

    // Removing `move` from closure's signature will cause closure
    // to borrow _haystack_ variable immutably, hence _haystack_ is still
    // available and uncommenting above line will not cause an error.
}
```

**b) As input parameters**
```rust
// A function which takes a closure as an argument and calls it.
// <F> denotes that F is a "Generic type parameter"
fn apply<F>(f: F) where
// The closure takes no input and returns nothing.
    F: FnOnce() {
    // ^ TODO: Try changing this to `Fn` or `FnMut`.

    f();
}

// A function which takes a closure and returns an `i32`.
fn apply_to_3<F>(f: F) -> i32 where
// The closure takes an `i32` and returns an `i32`.
    F: Fn(i32) -> i32 {

    f(3)
}

fn main() {
    use std::mem;

    let greeting = "hello";
    // A non-copy type.
    // `to_owned` creates owned data from borrowed one
    let mut farewell = "goodbye".to_owned();

    // Capture 2 variables: `greeting` by reference and
    // `farewell` by value.
    let diary = || {
        // `greeting` is by reference: requires `Fn`.
        println!("I said {}.", greeting);

        // Mutation forces `farewell` to be captured by
        // mutable reference. Now requires `FnMut`.
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // Manually calling drop forces `farewell` to
        // be captured by value. Now requires `FnOnce`.
        mem::drop(farewell);
    };

    // Call the function which applies the closure.
    apply(diary);

    // `double` satisfies `apply_to_3`'s trait bound
    let double = |x| 2 * x;
    println!("3 doubled: {}", apply_to_3(double));
}
```

**c) Type anonymity**
```rust
// `F` must implement `Fn` for a closure which takes no
// inputs and returns nothing - exactly what is required
// for `print`.
fn apply<F>(f: F) where
    F: Fn() {
    f();
}

fn main() {
    let x = 7;
    // Capture `x` into an anonymous type and implement
    // `Fn` for it. Store it in `print`.
    let print = || println!("{}", x);
    apply(print);
}
```

**d) Input functions**
```rust
// Define a function which takes a generic `F` argument
// bounded by `Fn`, and calls it
fn call_me<F: Fn()>(f: F) {
    f();
}

// Define a wrapper function satisfying the `Fn` bound
fn function() {
    println!("I'm a function!");
}

fn main() {
    // Define a closure satisfying the `Fn` bound
    let closure = || println!("I'm a closure!");

    call_me(closure);
    call_me(function);
}
```

**e) As output parameters**
```rust
fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();
    move || println!("This is a: {}", text)
}

fn create_fnmut() -> impl FnMut() {
    let text = "FnMut".to_owned();
    move || println!("This is a: {}", text)
}

fn create_fnonce() -> impl FnOnce() {
    let text = "FnOnce".to_owned();
    move || println!("This is a: {}", text)
}

fn main() {
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();
    fn_plain();
    fn_mut();
    fn_once();
}
```

**f) Examples in std**
```rust
fn main() {
    // 1. Iterator::any
    /*
    pub trait Iterator {
        // The type being iterated over.
        type Item;

        // `any` takes `&mut self` meaning the caller may be borrowed
        // and modified, but not consumed.
        fn any<F>(&mut self, f: F) -> bool where
            // `FnMut` meaning any captured variable may at most be
            // modified, not consumed. `Self::Item` states it takes
            // arguments to the closure by value.
            F: FnMut(Self::Item) -> bool {}
    }
    */
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];
    // `iter()` for vecs yields `&i32`. Destructure to `i32`.
    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
    // `into_iter()` for vecs yields `i32`. No destructuring required.
    println!("2 in vec2: {}", vec2.into_iter().any(| x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];
    // `iter()` for arrays yields `&i32`.
    println!("2 in array1: {}", array1.iter().any(|&x| x == 2));
    // `into_iter()` for arrays unusually yields `&i32`.
    println!("2 in array2: {}", array2.into_iter().any(|&x| x == 2));


    // 2. Searching through iterators
    /*
    pub trait Iterator {
        // The type being iterated over.
        type Item;

        // `find` takes `&mut self` meaning the caller may be borrowed
        // and modified, but not consumed.
        fn find<P>(&mut self, predicate: P) -> Option<Self::Item> where
            // `FnMut` meaning any captured variable may at most be
            // modified, not consumed. `&Self::Item` states it takes
            // arguments to the closure by reference.
            P: FnMut(&Self::Item) -> bool {}
    }
    */
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];
    // `iter()` for vecs yields `&i32`.
    let mut iter = vec1.iter();
    // `into_iter()` for vecs yields `i32`.
    let mut into_iter = vec2.into_iter();
    // `iter()` for vecs yields `&i32`, and we want to reference one of its
    // items, so we have to destructure `&&i32` to `i32`
    println!("Find 2 in vec1: {:?}", iter     .find(|&&x| x == 2));
    // `into_iter()` for vecs yields `i32`, and we want to reference one of
    // its items, so we have to destructure `&i32` to `i32`
    println!("Find 2 in vec2: {:?}", into_iter.find(| &x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];
    // `iter()` for arrays yields `&i32`
    println!("Find 2 in array1: {:?}", array1.iter()     .find(|&&x| x == 2));
    // `into_iter()` for arrays unusually yields `&i32`
    println!("Find 2 in array2: {:?}", array2.into_iter().find(|&&x| x == 2));

    let vec = vec![1, 9, 3, 3, 13, 2];
    let index_of_first_even_number = vec.iter().position(|x| x % 2 == 0);
    assert_eq!(index_of_first_even_number, Some(5));
    let index_of_first_negative_number = vec.iter().position(|x| x < &0);
    assert_eq!(index_of_first_negative_number, None);
}
```

#### 5.3. Higher Order Functions
```rust
fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

fn main() {
    println!("Find the sum of all the squared odd numbers under 1000");
    let upper = 1000;

    // Imperative approach
    // Declare accumulator variable
    let mut acc = 0;
    // Iterate: 0, 1, 2, ... to infinity
    for n in 0.. {
        // Square the number
        let n_squared = n * n;
        if n_squared >= upper {
            // Break loop if exceeded the upper limit
            break;
        } else if is_odd(n_squared) {
            // Accumulate value, if it's odd
            acc += n_squared;
        }
    }
    println!("imperative style: {}", acc);
    //=> imperative style: 5456

    // Functional approach
    let sum_of_squared_odd_numbers: u32 =
        (0..).map(|n| n * n)                                // All natural numbers squared
            .take_while(|&n_squared| n_squared < upper)     // Below upper limit
            .filter(|&n_squared| is_odd(n_squared))      // That are odd
            .fold(0, |acc, n_squared| acc + n_squared);           // Sum them
    println!("functional style: {}", sum_of_squared_odd_numbers);
    //=> functional style: 5456
}
```

#### 5.4. Diverging functions
```rust
fn main() {
    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            // Notice that the return type of this match expression must be u32
            // because of the type of the "addition" variable.
            let addition: u32 = match i%2 == 1 {
                // The "i" variable is of type u32, which is perfectly fine.
                true => i,
                // On the other hand, the "continue" expression does not return
                // u32, but it is still fine, because it never returns and therefore
                // does not violate the type requirements of the match expression.
                false => continue,
            };
            acc += addition;
        }
        acc
    }
    println!("Sum of odd numbers up to 9 (excluding): {}", sum_odd_numbers(9));
}
```

### 6. Modules
#### 6.1. Visibility
```rust
// A module named my_mod
mod my_mod {
    // Items in modules default to private visibility.
    fn private_function() {
        println!("called my_mod::private_function()");
    }

    // Use the pub modifier to override default visibility.
    pub fn function() {
        println!("called my_mod::function()");
    }

    // Items can access other items in the same module,
    // even when private.
    pub fn indirect_access() {
        print!("called my_mod::indirect_access(), that\n");
        private_function();
    }

    // Modules can also be nested
    pub mod nested {
        pub fn function() {
            println!("called my_mod::nested::function()");
        }

        #[allow(dead_code)]
        fn private_funtion() {
            println!("called my_mod::nested::private_function()");
        }

        // Function declared using pub(in path) syntax are only visible
        // within the given path. Path must be a parent or ancestor module
        pub(in crate::my_mod) fn public_function_in_my_mod() {
            print!("called my_mod::nested::public_function_in_my_mod(), that\n");
            public_function_in_nested();
        }

        // Functions declared using pub(self) syntax are only visible within
        // the current module, which is the same as leaving them private
        pub(self) fn public_function_in_nested() {
            println!("called my_mod::nested::public_function_in_nested()");
        }

        // Function declared using pub(super) syntax are only visible within
        // the parent module
        pub(super) fn public_function_in_super_mod() {
            println!("called my_mod::nested::public_function_in_super_mod()");
        }
    }

    pub fn call_public_function_in_my_mod() {
        print!("called my_mod::call_public_function_in_my_mod(), that\n");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    // pub(crate) makes functions visible only within the current crate
    pub(crate) fn public_function_in_crate() {
        println!("called my_mod::public_function_in_crate()");
    }

    // Nested modules follow the same rules for visibility
    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called my_mod::private_nested::function()");
        }

        // Private parent items will still restrict the visibility of a child item,
        // even if it is declared as visible within a bigger scope.
        pub(crate) fn restricted_function() {
            println!("called my_mod::private_nested::restricted_function()");
        }
    }
}

fn function() {
    println!("called function()");
}

fn main() {
    // Module allow disambiguation between items that have the same name.
    function();
    my_mod::function();

    // Public items, including those inside nested modules, can be
    // access from outside the parent module.
    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();

    // pub(crate) items can be called from anywhere in the same crate
    my_mod::public_function_in_crate();

    // pub(in path) items can only be called from within the module specified
    // Error! function `public_function_in_my_mod` is private
    // my_mod::nested::public_function_in_my_mod();
    // TODO ^ Try uncommenting this line

    // Private items of a module cannot be directly accessed, even if
    // nested in a public module:

    // Error! `private_function` is private
    // my_mod::private_function();
    // TODO ^ Try uncommenting this line

    // Error! `private_function` is private
    // my_mod::nested::private_function();
    // TODO ^ Try uncommenting this line

    // Error! `private_nested` is a private module
    // my_mod::private_nested::function();
    // TODO ^ Try uncommenting this line

    // Error! `private_nested` is a private module
    // my_mod::private_nested::restricted_function();
    // TODO ^ Try uncommenting this line
}
```

#### 6.2. Struct visibility
```rust
mod my {
    // A public struct with a public field of generic type T
    pub struct OpenBox<T> {
        pub contents: T,
    }

    // A public struct with a private field of generic type T
    #[allow(dead_code)]
    pub struct ClosedBox<T> {
        contents: T,
    }

    impl<T> ClosedBox<T> {
        // A public constructor method
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox {
                contents,
            }
        }
    }
}

fn main() {
    // Public structs with public fields can be constructed as usual
    let open_box = my::OpenBox {contents: "public information"};

    // and their fields can be normally accessed.
    println!("The open box contains: {}", open_box.contents);

    // Public structs with private fields cannot be constructed using field names.
    // Error! `ClosedBox` has private fields
    // let closed_box = my::ClosedBox { contents: "classified information" };
    // TODO ^ Try uncommenting this line

    // However, structs with private fields can be created using
    // public constructors
    let _close_box = my::ClosedBox::new("classified information");

    // and the private fields of a public struct cannot be accessed.
    // Error! The `contents` field is private
    // println!("The closed box contains: {}", _closed_box.contents);
    // TODO ^ Try uncommenting this line
}
```

#### 6.3. The use declaration
```rust
// Bind the deeply::nested::function path to other_function.
use deeply::nested::function as other_function;

fn function() {
    println!("called function()");
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called deeply::nested::function()");
        }
    }
}

fn main() {
    // Easier access to deeply::nested::function
    other_function();
    println!("Entering block");
    {
        // This is equivalent to use deeply::nested::function as function.
        // This function() will shadow the outer one.
        use crate::deeply::nested::function;
        // use binmding have a local scope. In this case,
        // the shadowing of function() is only in this block.
        function();

        println!("Leaving block");
    }
    function();
    //=> called deeply::nested::function()
    //=> Entering block
    //=> called deeply::nested::function()
    //=> Leaving block
    //=> called function()
}
```

#### 6.4. super and self
```rust
fn function() {
    println!("called function()");
}

mod cool {
    pub fn function() {
        println!("called cool::function()");
    }
}

mod my {
    fn function() {
        println!("called my::function()");
    }
    mod cool {
        pub fn function() {
            println!("called my::cool::function()");
        }
    }
    pub fn indirect_call() {
        // Let't access all the function named function from this scope!
        print!("called my::indirect_call, that\n");
        // The self keyword refers to the current module scope - in this case my.
        // Calling self::function() and calling function() directly both give
        // the same result, because they refer to the same function.
        self::function();
        function();
        // We can also use self to access another module inside my:
        self::cool::function();
        // The super keyword refers to the parent scope (outside the my module).
        super::function();
        // This will bind to the cool::function in the crate scope.
        // In this case the crate scope is the outermost scope.
        {
            use crate::cool::function as root_function;
            root_function();
        }
    }
}

fn main() {
    my::indirect_call();
    //=> called my::indirect_call, that
    //=> called my::function()
    //=> called my::function()
    //=> called my::cool::function()
    //=> called function()
    //=> called cool::function()
}
```

#### 6.5. File hierarchy
```rust
// Modules can be mapped to a file/directory hierarchy.
/*
tree .
.
|-- my
|   |-- inaccessible.rs
|   |-- mod.rs
|   `-- nested.rs
`-- split.rs
*/
// file split.rs
mod my;
fn function() {
    println!("called `function()`");
}
fn main() {
    my::function();
    function();
    my::indirect_access();
    my::nested::function();
}
// rustc split.rs && ./split
```

### 7. Crates
#### 7.1. Creating a Library
```rust
// file rary.rs
pub fn public_function() {
    println!("called rary's `public_function()`");
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");
    private_function();
}
// compile
// rustc --crate-type=lib rary.rs
```

#### 7.2. Using a Library
```rust
// file executable.rs
fn main() {
    rary::public_function();
    // Error! `private_function` is private
    //rary::private_function();
    rary::indirect_access();
}
// compile & run
// rustc executable.rs --extern rary=library.rlib --edition=2018 && ./executable
// called rary's `public_function()`
// called rary's `indirect_access()`, that
// > called rary's `private_function()`
```

### 8. Cargo
**cargo** is the official Rust package management tool.  
Read more at [The Cargo Book](https://doc.rust-lang.org/cargo/).  

### 9. Attributes
An attribute is metadata applied to some module, crate or item. This metadata can be used to/for:
+ conditional compilation of code
+ set crate name, version and type (binary or library)
+ disable lints (warnings)
+ enable compiler features (macros, glob imports, etc.)
+ link to a foreign library
+ mark functions as unit tests
+ mark functions that will be part of a benchmark

### 10. Generics
**Generics** is the topic of generalizing types and functionalities to broader cases.  
```rust
// A concrete type A.
struct A;

// In defining the type Single, the first use of A is not preceded by <A>.
// Therefore, Single is a concrete type, and A is defined as above.
struct Single<A>;
//            ^ Here is `Single`s first use of the type `A`.

// Here, <T> precedes the first use of T, so SingleGen is a generic type.
// Because the type parameter T is generic, it could be anything, including
// the conrete type A defined at the top.
struct SingleGen<T>(T);

fn main() {
    // Single is concrete and explicitly takes A.
    let _s = Single(A);

    // Create a variable _char of type SingleGen<char>
    // and give it the value SIngleGen('a').
    // Here, SingleGen has a type parameter explicitly specified.
    let _char: SingleGen<char> = SingleGen('a');

    // SingleGen can also have a type parameter implicitly specified:
    let _t = SingleGen(A); // Use A defined at the top.
    let _i32 = SingleGen(6); // Uses i32.
    let _char = SingleGen('a'); // Uses char.
}
```

### 11. Traits
A **trait** is a collection of methods defined for an unknown type: Self. They can access other methods declared in the same trait.  
```rust
struct Sheep {naked: bool, name: &'static str}

trait Animal {
    // Static method signature; Self refers to the implementor type.
    fn new(name: &'static str) -> Self;
    // Instance method signatures; these will return a string.
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;
    // Traits can provide default method definitions.
    fn talk(&self) {
        println!("{} say {}", self.name(), self.noise());
    }
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }
    fn shear(&mut self) {
        if self.is_naked() {
            // Implementor methods can use the implementor's trait methods.
            println!("{} is already naked...", self.name());
        } else {
            println!("{} gets a haircut!", self.name);
            self.naked = true;
        }
    }
}

// Implement the Animal trait for Sheep.
impl Animal for Sheep {
    // Self is the implementor type: Sheep.
    fn new(name: &'static str) -> Self {
        Sheep {name: name, naked: false}
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baaaaah?"
        } else {
            "baaaaah!"
        }
    }

    // Default trait methods can be overridden.
    fn talk(&self) {
        // For example, we can add some quiet comtemplation.
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}

fn main() {
    // Type annotation is necessary in this case.
    let mut dolly:Sheep = Animal::new("Dolly");
    // TODO ^ Try removing the type annotations.
    dolly.talk();
    dolly.shear();
    dolly.talk();
    //=> Dolly pauses briefly... baaaaah!
    //=> Dolly gets a haircut!
    //=> Dolly pauses briefly... baaaaah?
}
```

#### 11.1. Derive
```rust
// `Centimeters`, a tuple struct that can be compared
#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

// `Inches`, a tuple struct that can be printed
#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;
        Centimeters(inches as f64 * 2.54)
    }
}

// `Seconds`, a tuple struct with no additional attributes
struct Seconds(i32);

fn main() {
    let _one_second = Seconds(1);
    // Error: `Seconds` can't be printed; it doesn't implement the `Debug` trait
    // println!("One second looks like: {:?}", _one_second);
    // TODO ^ Try uncommenting this line
    // Error: `Seconds` can't be compared; it doesn't implement the `PartialEq` trait
    // let _this_is_true = (_one_second == _one_second);
    // TODO ^ Try uncommenting this line
    let foot = Inches(12);
    println!("One foot equals {:?}", foot);
    let meter = Centimeters(100.0);
    let cmp =
        if foot.to_centimeters() < meter {
            "smaller"
        } else {
            "bigger"
        };
    println!("One foot is {} than one meter.", cmp);
    //=> One foot equals Inches(12)
    //=> One foot is smaller than one meter.
}
```

#### 11.2. Returning Traits with dyn
```rust
struct Sheep {}
struct Cow {}

trait Animal {
    // Instance method signature
    fn noise(&self) -> &'static str;
}

// Implement the `Animal` trait for `Sheep`.
impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "baaaaah!"
    }
}

// Implement the `Animal` trait for `Cow`.
impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "moooooo!"
    }
}

// Returns some struct that implements Animal, but we don't know which one at compile time.
fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

fn main() {
    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!("You've randomly chosen an animal, and it says {}", animal.noise());
    //=> You've randomly chosen an animal, and it says baaaaah!
}
```

#### 11.3. Operator Overloading
```rust
use std::ops;

struct Foo;
struct Bar;

#[derive(Debug)]
struct FooBar;

#[derive(Debug)]
struct BarFoo;

// The `std::ops::Add` trait is used to specify the functionality of `+`.
// Here, we make `Add<Bar>` - the trait for addition with a RHS of type `Bar`.
// The following block implements the operation: Foo + Bar = FooBar
impl ops::Add<Bar> for Foo {
    type Output = FooBar;
    fn add(self, _rhs: Bar) -> FooBar {
        println!("> Foo.add(Bar) was called");
        FooBar
    }
}

// By reversing the types, we end up implementing non-commutative addition.
// Here, we make `Add<Foo>` - the trait for addition with a RHS of type `Foo`.
// This block implements the operation: Bar + Foo = BarFoo
impl ops::Add<Foo> for Bar {
    type Output = BarFoo;
    fn add(self, _rhs: Foo) -> BarFoo {
        println!("> Bar.add(Foo) was called");
        BarFoo
    }
}

fn main() {
    println!("Foo + Bar = {:?}", Foo + Bar);
    println!("Bar + Foo = {:?}", Bar + Foo);
    // > Foo.add(Bar) was called
    // Foo + Bar = FooBar
    // > Bar.add(Foo) was called
    // Bar + Foo = BarFoo
}
```

#### 11.4. Drop
```rust
struct Droppable {
    name: &'static str,
}

// This trivial implementation of `drop` adds a print to console.
impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> Dropping {}", self.name);
    }
}

fn main() {
    let _a = Droppable { name: "a" };
    // block A
    {
        let _b = Droppable { name: "b" };
        // block B
        {
            let _c = Droppable { name: "c" };
            let _d = Droppable { name: "d" };
            println!("Exiting block B");
        }
        println!("Just exited block B");
        println!("Exiting block A");
    }
    println!("Just exited block A");
    // Variable can be manually dropped using the `drop` function
    drop(_a);
    // TODO ^ Try commenting this line
    println!("end of the main function");
    // `_a` *won't* be `drop`ed again here, because it already has been
    // (manually) `drop`ed

    // Exiting block B
    // > Dropping d
    // > Dropping c
    // Just exited block B
    // Exiting block A
    // > Dropping b
    // Just exited block A
    // > Dropping a
    // end of the main function
}
```

#### 11.5. Iterators
```rust
struct Fibonacci {
    curr: u32,
    next: u32,
}

// Implement `Iterator` for `Fibonacci`.
// The `Iterator` trait only requires a method to be defined for the `next` element.
impl Iterator for Fibonacci {
    type Item = u32;
    // Here, we define the sequence using `.curr` and `.next`.
    // The return type is `Option<T>`:
    //     * When the `Iterator` is finished, `None` is returned.
    //     * Otherwise, the next value is wrapped in `Some` and returned.
    fn next(&mut self) -> Option<u32> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;
        // Since there's no endpoint to a Fibonacci sequence, the `Iterator`
        // will never return `None`, and `Some` is always returned.
        Some(self.curr)
    }
}

// Returns a Fibonacci sequence generator
fn fibonacci() -> Fibonacci {
    Fibonacci{curr: 0, next: 1}
}

fn main() {
    // `0..3` is an `Iterator` that generates: 0, 1, and 2.
    let mut sequence = 0..3;
    println!("Four consecutive next calls on 0..3");
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    // `for` works through an `Iterator` until it returns `None`.
    // Each `Some` value is unwrapped and bound to a variable (here, `i`).
    println!("Iterate through 0..3 using for");
    for i in 0..3 {
        println!("> {}", i);
    }
    // The `skip(n)` method shortens an `Iterator` by dropping its first `n` terms.
    println!("The next four terms of the Fibonacci sequence are: ");
    for i in fibonacci().skip(4).take(4) {
        println!("> {}", i);
    }
    let array = [1u32, 3, 3, 7];
    // The `iter` method produces an `Iterator` over an array/slice.
    println!("Iterate the following array {:?}", &array);
    for i in array.iter() {
        println!("> {}", i);
    }
}
```

#### 11.6. impl Trait
```rust
use std::iter;
use std::vec::IntoIter;

// This function combines two `Vec<i32>` and returns an iterator over it.
// Look how complicated its return type is!
#[allow(dead_code)]
fn combine_vecs_explicit_return_type(v: Vec<i32>, u: Vec<i32>) ->
iter::Cycle<iter::Chain<IntoIter<i32>, IntoIter<i32>>> {
    v.into_iter().chain(u.into_iter()).cycle()
}

// This is the exact same function, but its return type uses `impl Trait`.
// Look how much simpler it is!
fn combine_vecs(v: Vec<i32>, u: Vec<i32>) -> impl Iterator<Item=i32> {
    v.into_iter().chain(u.into_iter()).cycle()
}

// Returns a function that adds `y` to its input
fn make_adder_function(y: i32) -> impl Fn(i32) -> i32 {
    let closure = move |x: i32| { x + y };
    closure
}

#[allow(dead_code)]
fn double_positives<'a>(numbers: &'a Vec<i32>) -> impl Iterator<Item = i32> + 'a {
    numbers
        .iter()
        .filter(|x| x > &&0)
        .map(|x| x * 2)
}

fn main() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5];
    let mut v3 = combine_vecs(v1, v2);
    assert_eq!(Some(1), v3.next());
    assert_eq!(Some(2), v3.next());
    assert_eq!(Some(3), v3.next());
    assert_eq!(Some(4), v3.next());
    assert_eq!(Some(5), v3.next());
    println!("all done");

    let plus_one = make_adder_function(1);
    assert_eq!(plus_one(2), 3);
}
```

#### 11.7. Clone
```rust
// A unit struct without resources
#[derive(Debug, Clone, Copy)]
struct Unit;

// A tuple struct with resources that implements the `Clone` trait
#[derive(Clone, Debug)]
struct Pair(Box<i32>, Box<i32>);

fn main() {
    // Instantiate `Unit`
    let unit = Unit;
    // Copy `Unit`, there are no resources to move
    let copied_unit = unit;

    // Both `Unit`s can be used independently
    println!("original: {:?}", unit);
    println!("copy: {:?}", copied_unit);

    // Instantiate `Pair`
    let pair = Pair(Box::new(1), Box::new(2));
    println!("original: {:?}", pair);

    // Move `pair` into `moved_pair`, moves resources
    let moved_pair = pair;
    println!("moved: {:?}", moved_pair);

    // Error! `pair` has lost its resources
    //println!("original: {:?}", pair);
    // TODO ^ Try uncommenting this line

    // Clone `moved_pair` into `cloned_pair` (resources are included)
    let cloned_pair = moved_pair.clone();
    // Drop the original pair using std::mem::drop
    drop(moved_pair);

    // Error! `moved_pair` has been dropped
    //println!("copy: {:?}", moved_pair);
    // TODO ^ Try uncommenting this line

    // The result from .clone() can still be used!
    println!("clone: {:?}", cloned_pair);
}
```

#### 11.8. Supertraits
```rust
trait Person {
    fn name(&self) -> String;
}

// Person is a supertrait of Student.
// Implementing Student requires you to also impl Person.
trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

// CompSciStudent (computer science student) is a subtrait of both Programmer 
// and Student. Implementing CompSciStudent requires you to impl both supertraits.
trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}

fn main() {}
```

#### 11.9. Disambiguating overlapping traits
```rust
trait UsernameWidget {
    // Get the selected username out of this witget
    fn get(&self) -> String;
}

trait AgeWidget {
    // Get the selected age out of this widget
    fn get(&self) -> u8;
}

// A form with both a UsernameWidget and an AgeWidget
struct Form {
    username: String,
    age: u8,
}

impl UsernameWidget for Form {
    fn get(&self) -> String {
        self.username.clone()
    }
}

impl AgeWidget for Form {
    fn get(&self) -> u8 {
        self.age
    }
}

fn main() {
    let form = Form {
        username: "rustacean".to_owned(),
        age: 28,
    };
    // If you uncomment this line, you'll get an error saying
    // "multiple `get` found". Because, after all, there are multiple methods
    // named `get`.
    // println!("{}", form.get());
    let username = <Form as UsernameWidget>::get(&form);
    assert_eq!("rustacean".to_owned(), username);
    let age = <Form as AgeWidget>::get(&form);
    assert_eq!(28, age);
}
```

### 12. Std library types
The **std** library provides many custom types which expands drastically on the primitives. Some of these include:  
+ growable Strings like: "hello world"
+ growable vectors: [1, 2, 3]
+ optional types: Option<i32>
+ error handling types: Result<i32, i32>
+ heap allocated pointers: Box<i32>

#### 12.1. Box, stack and heap
All values in Rust are stack allocated by default. Values can be boxed (allocated on the heap) by creating a Box<T>. A box is a smart pointer to a heap allocated value of type T.  
Boxed values can be dereferenced using the * operator; this removes one layer of indirection.  
```rust
use std::mem;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

// A Rectangle can be specified by where its top left and bottom right
// corners are in space
#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn origin() -> Point {
    Point {x: 0.0, y: 0.0}
}

fn boxed_origin() -> Box<Point> {
    // Allocate this point on the heap, and return a pointer to it
    Box::new(Point{x: 0.0, y: 0.0})
}

fn main() {
    // (all the type annotations are superfluous)
    // Stack allocated variables
    let point: Point = origin();
    let rectangle: Rectangle = Rectangle{
        top_left: origin(),
        bottom_right: Point{x: 3.0, y: -4.0},
    };
    // Heap allocated rectangle
    let boxed_rectangle: Box<Rectangle> = Box::new(Rectangle{
        top_left: origin(),
        bottom_right: Point{x: 3.0, y: -4.0},
    });
    // The output of functions can be boxed
    let boxed_point: Box<Point> = Box::new(origin());
    // Double indirection
    let box_in_a_box: Box<Box<Point>> = Box::new(boxed_origin());
    println!("Point occupies {} bytes on the stack",
             mem::size_of_val(&point));
    println!("Rectangle occupies {} bytes on the stack",
             mem::size_of_val(&rectangle));
    // box size == pointer size
    println!("Boxed point occupies {} bytes on the stack",
             mem::size_of_val(&boxed_point));
    println!("Boxed rectangle occupies {} bytes on the stack",
             mem::size_of_val(&boxed_rectangle));
    println!("Boxed box occupies {} bytes on the stack",
             mem::size_of_val(&box_in_a_box));
    // Copy the data contained in boxed_point into unboxed_point
    let unboxed_point: Point = *boxed_point;
    println!("Unboxed point occupies {} bytes on the stack",
            mem::size_of_val(&unboxed_point));
    // Point occupies 16 bytes on the stack
    // Rectangle occupies 32 bytes on the stack
    // Boxed point occupies 8 bytes on the stack
    // Boxed rectangle occupies 8 bytes on the stack
    // Boxed box occupies 8 bytes on the stack
    // Unboxed point occupies 16 bytes on the stack
}
```

#### 12.2. Vectors
Vectors are re-sizable arrays. A vector is represented using 3 parameters:  
+ pointer to the data
+ length
+ capacity
```rust
fn main() {
    // Iterators can be collected into vectors
    let collected_iterator: Vec<i32> = (0..10).collect();
    println!("Collected (0..10) into: {:?}", collected_iterator);
    // The vec! macro can be used to initialize a vector
    let mut xs = vec![1i32, 2, 3];
    println!("Initial vector: {:?}", xs);
    // Insert new element at the end of the vector
    println!("Push 4 into the vector");
    xs.push(4);
    println!("Vector: {:?}", xs);
    // Error! Immutable vectors can't grow
    // collected_iterator.push(0);
    // FIXME ^ Comment out this line
    // The len method yeaids the number of elements currently storied in a vector
    println!("Vector length: {}", xs.len());
    // Indexing is done using the square brackets (indexing starts at 0)
    println!("Second element: {}", xs[1]);
    // `pop` removes the last element from the vector and returns it
    println!("Pop last element: {:?}", xs.pop());
    //=> Pop last element: Some(4)
    // Out of bounds indexing yields a panic
    // println!("Fourth element: {}", xs[3]);
    // FIXME ^ Comment out this line
    // Vector can be easily iterated over
    println!("Contents of xs:");
    for x in xs.iter() {
        println!("> {}", x);
    }
    // A Vector can also be iterated over while the iteration
    // count is enumerated in a separate variable (i)
    for (i,x) in xs.iter().enumerate() {
        println!("In position {} we have value {}", i, x);
    }
    // Thanks to iter_mut, mutable Vector can also be iterated
    // over in a way that allows modifying each value
    for x in xs.iter_mut() {
        *x *= 3;
    }
    println!("Updated vector: {:?}", xs);
    //=> Updated vector: [3, 6, 9]
}
```

#### 12.3. Strings
There are two types of strings in Rust: String and &str.  
A **String** is stored as a vector of bytes (Vec<u8>), but guaranteed to always be a valid UTF-8 sequence. String is heap allocated, growable and not null terminated.  
**&str** is a slice (&[u8]) that always points to a valid UTF-8 sequence, and can be used to view into a String, just like &[T] is a view into Vec<T>.  
```rust
use std::str;

fn main() {
    // 1. Basic
    // (All the type annotations are superfluous)
    // A reference to a string allocated in read only memory
    let pangram: &'static str = "The quick brown fox jumps over the lazy dog";
    println!("Pangram: {}", pangram);
    // Iterate over words in reverse, no new string is allocated
    println!("Words in reverse");
    for word in pangram.split_whitespace().rev() {
        println!("> {}", word);
    }
    // Copy chars into a vector, sort and remove duplicates.
    let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort();
    chars.dedup();
    println!("chars: {:?}", chars);
    // Create an empty and growable String
    let mut string = String::new();
    for c in chars {
        // Insert a char at the end of string
        string.push(c);
        // Insert a string at the end of string
        string.push_str(", ");
    }
    println!("string: {}", string);
    // The trimmed string is a slide to the original string, hence no new
    // allocation is performed
    let chars_to_trim: &[char] = &[' ', ','];
    let trimmed_str: &str = string.trim_matches(chars_to_trim);
    println!("Used characters: {}", trimmed_str);
    // Heap allocate a string
    let alice = String::from("I like dogs");
    // Allocate new memory and store the modified string there
    let bob: String = alice.replace("dog", "cat");
    println!("Alice says: {}", alice);
    println!("Bob says: {}", bob);
    //=> Alice says: I like dogs
    //=> Bob says: I like cats

    // 2. Literals and escapes
    // You can use escapes to write bytes by their hexadecimal values...
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);
    // ...or Unicode code points.
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";
    println!("Unicode character {} (U+211D) is called {}",
             unicode_codepoint, character_name );
    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here ->\
                        <- can be escaped too!";
    println!("{}", long_string);
    // What are you doing? (\x3F means ?) I'm writing Rust!
    // Unicode character ℝ (U+211D) is called "DOUBLE-STRUCK CAPITAL R"
    // String literals
    // can span multiple lines.
    //     The linebreak and indentation here -><- can be escaped too!

    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);
    // If you need quotes in a raw string, add a pair of #s
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);
    // If you need "# in your string, just use more #s in the delimiter.
    // There is no limit for the number of #s you can use.
    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter);
    // Escapes don't work here: \x3F \u{211D}
    // And then I said: "There is no escape!"
    // A string with "# in it. And even "##!

    // Note that this is not actually a `&str`
    let bytestring: &[u8; 21] = b"this is a byte string";
    // Byte arrays don't have the `Display` trait, so printing them is a bit limited
    println!("A byte string: {:?}", bytestring);
    // Byte strings can have byte escapes...
    let escaped = b"\x52\x75\x73\x74 as bytes";
    // ...but no unicode escapes
    // let escaped = b"\u{211D} is not allowed";
    println!("Some escaped bytes: {:?}", escaped);

    // Raw byte strings work just like raw strings
    let raw_bytestring = br"\u{211D} is not escaped here";
    println!("{:?}", raw_bytestring);
    // Converting a byte array to `str` can fail
    if let Ok(my_str) = str::from_utf8(raw_bytestring) {
        println!("And the same as text: '{}'", my_str);
    }
    let _quotes = br#"You can also use "fancier" formatting, \
                    like with normal raw strings"#;
    // Byte strings don't have to be UTF-8
    let shift_jis = b"\x82\xe6\x82\xa8\x82\xb1\x82\xbb"; // "ようこそ" in SHIFT-JIS
    // But then they can't always be converted to `str`
    match str::from_utf8(shift_jis) {
        Ok(my_str) => println!("Conversion successful: '{}'", my_str),
        Err(e) => println!("Conversion failed: {:?}", e),
    };
    //=> Conversion failed: Utf8Error { valid_up_to: 0, error_len: Some(1) }
}
```

#### 12.4. Option
Sometimes it's desirable to catch the failure of some parts of a program instead of calling panic!; this can be accomplished using the Option enum.  
The `Option<T>` enum has two variants:
+ **None**, to indicate failure or lack of value, and
+ **Some(value)**, a tuple struct that wraps a value with type T.
```rust
// An integer division that doesn't `panic!`
fn checked_division(dividend: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
        // Failure is represented as the `None` variant
        None
    } else {
        // Result is wrapped in a `Some` variant
        Some(dividend / divisor)
    }
}

// This function handles a division that may not succeed
fn try_division(dividend: i32, divisor: i32) {
    // `Option` values can be pattern matched, just like other enums
    match checked_division(dividend, divisor) {
        None => println!("{} / {} failed!", dividend, divisor),
        Some(quotient) => {
            println!("{} / {} = {}", dividend, divisor, quotient)
        },
    }
}

fn main() {
    try_division(4, 2);
    try_division(1, 0);
    // Binding `None` to a variable needs to be type annotated
    let none: Option<i32> = None;
    let _equivalent_none = None::<i32>;
    let optional_float = Some(0f32);
    // Unwrapping a `Some` variant will extract the value wrapped.
    println!("{:?} unwraps to {:?}", optional_float, optional_float.unwrap());
    // Unwrapping a `None` variant will `panic!`
    println!("{:?} unwraps to {:?}", none, none.unwrap());
}
```

#### 12.4. Result
The `Result<T, E>` enum has two variants:  
+ `Ok(value)` which indicates that the operation succeeded, and wraps the `value` returned by the operation. (`value` has type `T`)
+ `Err(why)`, which indicates that the operation failed, and wraps `why`, which (hopefully) explains the cause of the failure. (`why` has type `E`)
```rust
mod checked {
    // Mathematical "errors" we want to catch
    #[derive(Debug)]
    pub enum MathError {
        DivisionByZero,
        NonPositiveLogarithm,
        NegativeSquareRoot,
    }

    pub type MathResult = Result<f64, MathError>;

    pub fn div(x: f64, y: f64) -> MathResult {
        if y == 0.0 {
            // This operation would `fail`, instead let's return the reason of
            // the failure wrapped in `Err`
            Err(MathError::DivisionByZero)
        } else {
            // This operation is valid, return the result wrapped in `Ok`
            Ok(x / y)
        }
    }

    pub fn sqrt(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }

    pub fn ln(x: f64) -> MathResult {
        if x <= 0.0 {
            Err(MathError::NonPositiveLogarithm)
        } else {
            Ok(x.ln())
        }
    }

    // Intermediate function
    fn op_(x: f64, y: f64) -> MathResult {
        // if `div` "fails", then `DivisionByZero` will be `return`ed
        let ratio = div(x, y)?;
        // if `ln` "fails", then `NonPositiveLogarithm` will be `return`ed
        let ln = ln(ratio)?;
        sqrt(ln)
    }

    pub fn op(x: f64, y: f64) {
        match op_(x, y) {
            Err(why) => panic!(match why {
                MathError::NonPositiveLogarithm
                => "logarithm of non-positive number",
                MathError::DivisionByZero
                => "division by zero",
                MathError::NegativeSquareRoot
                => "square root of negative number",
            }),
            Ok(value) => println!("{}", value),
        }
    }
}

// `op(x, y)` === `sqrt(ln(x / y))`
fn op(x: f64, y: f64) -> f64 {
    // This is a three level match pyramid!
    match checked::div(x, y) {
        Err(why) => panic!("{:?}", why),
        Ok(ratio) => match checked::ln(ratio) {
            Err(why) => panic!("{:?}", why),
            Ok(ln) => match checked::sqrt(ln) {
                Err(why) => panic!("{:?}", why),
                Ok(sqrt) => sqrt,
            },
        },
    }
}

fn main() {
    // Will this fail?
    println!("{}", op(1.0, 10.0));
    checked::op(1.0, 10.0);
}
```

#### 12.5. HashMap & HashSet
**HashMap** is a structure Key-Value.  
**HashSet** as a HashMap where we just care about the keys (`HashSet<T>` is, in actuality, just a wrapper around `HashMap<T, ()>`).  
**Sets** have 4 primary operations (all of the following calls return an iterator):  
+ **union**: get all the unique elements in both sets.
+ **difference**: get all the elements that are in the first set but not the second.
+ **intersection**: get all the elements that are only in both sets.
+ **symmetric_difference**: get all the elements that are in one set or the other, but not both.
```rust
use std::collections::HashMap;
use std::collections::HashSet;

fn call(number: &str) -> &str {
    match number {
        "798-1364" => "We're sorry, the call cannot be completed as dialed.
            Please hang up and try again.",
        "645-7689" => "Hello, this is Mr. Awesome's Pizza. My name is Fred.
            What can I get for you today?",
        _ => "Hi! Who is this again?"
    }
}

fn main() {
    // 1. HashMap
    let mut contacts = HashMap::new();
    contacts.insert("Daniel", "798-1364");
    contacts.insert("Ashley", "645-7689");
    contacts.insert("Katie", "435-8291");
    contacts.insert("Robert", "956-1745");
    // Take a reference and returns Option<&V>
    match contacts.get(&"Daniel") {
        Some(&number) => println!("Calling Daniel: {}", call(number)),
        _ => println!("Don't have Daniel's number."),
    }
    // HashMap::insert() returns Nome
    // If the inserted value is new, Some(value) otherwise
    contacts.insert("Daniel", "164-6743");
    match contacts.get(&"Ashley") {
        Some(&number) => println!("Calling Ashley: {}", call(number)),
        _ => println!("Don't have Ashley's number."),
    }
    contacts.remove(&"Ashley");
    // HashMap::iter() returns an iterator that yields
    // (&'a key, &'a value) pair in arbitrary order.
    for (contact, &number) in contacts.iter() {
        println!("Calling {}: {}", contact, call(number));
    }

    // 2. HashSet
    let mut a: HashSet<i32> = vec![1i32, 2, 3].into_iter().collect();
    let mut b: HashSet<i32> = vec![2i32, 3, 4].into_iter().collect();
    assert!(a.insert(4));
    assert!(a.contains(&4));
    // HashSet::insert() returns false if
    // there was a value already present.
    // assert!(b.insert(4), "Value 4 is already in set B!");
    // FIXME ^ Comment out this line
    b.insert(5);
    // If a collection's element type implements `Debug`,
    // then the collection implements `Debug`.
    // It usually prints its elements in the format `[elem1, elem2, ...]`
    println!("A: {:?}", a);
    println!("B: {:?}", b);
    // Print [1, 2, 3, 4, 5] in arbitrary order
    println!("Union: {:?}", a.union(&b).collect::<Vec<&i32>>());
    // This should print [1]
    println!("Difference: {:?}", a.difference(&b).collect::<Vec<&i32>>());
    // Print [2, 3, 4] in arbitrary order.
    println!("Intersection: {:?}", a.intersection(&b).collect::<Vec<&i32>>());
    // Print [1, 5]
    println!("Symmetric Difference: {:?}", a.symmetric_difference(&b).collect::<Vec<&i32>>());
}
```

#### 12.6. Rc
When multiple ownership is needed, `Rc`(Reference Counting) can be used. Rc keeps track of the number of the references which means the number of owners of the value wrapped inside an `Rc`.  
```rust
use std::rc::Rc;

fn main() {
    let rc_examples = "Rc examples".to_string();
    {
        println!("--- rc_a is created ---");
        let rc_a: Rc<String> = Rc::new(rc_examples);
        println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));
        {
            println!("--- rc_a is cloned to rc_b ---");
            let rc_b: Rc<String> = Rc::clone(&rc_a);
            println!("Reference Count of rc_b: {}", Rc::strong_count(&rc_b));
            println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));
            // Two `Rc`s are equal if their inner values are equal
            println!("rc_a and rc_b are equal: {}", rc_a.eq(&rc_b));
            // We can use methods of a value directly
            println!("Length of the value inside rc_a: {}", rc_a.len());
            println!("Value of rc_b: {}", rc_b);
            println!("--- rc_b is dropped out of scope ---");
        }
        println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));
        println!("--- rc_a is dropped out of scope ---");
    }
    // Error! `rc_examples` already moved into `rc_a`
    // And when `rc_a` is dropped, `rc_examples` is dropped together
    // println!("rc_examples: {}", rc_examples);
    // TODO ^ Try uncommenting this line
}
```

#### 12.7. Arc
When shared ownership between threads is needed, `Arc`(Atomic Reference Counted) can be used.  
```rust
fn main() {
    use std::sync::Arc;
    use std::thread;

    // This variable declaration is where it's value is specified.
    let apple = Arc::new("the same apple");
    for _ in 0..10 {
        // Here there is no value specification as it is a pointer to a reference
        // in the memory heap.
        let apple = Arc::clone(&apple);
        thread::spawn(move || {
            // As Arc was used, threads can be spawned using the value allocated
            // in the Arc variable pointer's location.
            println!("{:?}", apple);
        });
    }
}
```

### 13. Std misc
##### 13.1. Threads
Rust provides a mechanism for spawning native OS threads via the `spawn` function, the argument of this function is a moving closure.  
```rust
use std::thread;
const NTHREADS: u32 = 10;
// This is the main thread
fn main() {
    // Make a vector to hold the children which are spawned.
    let mut children = vec![];
    for i in 0..NTHREADS {
        // Spin up another thread
        children.push(thread::spawn(move || {
            println!("this is thread number {}", i);
        }));
    }
    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }
    println!("Done!!!...");
}
```
**Testcase: map-reduce**
```rust
use std::thread;

// This is the `main` thread
fn main() {
    // This is our data to process.
    // We will calculate the sum of all digits via a threaded  map-reduce algorithm.
    // Each whitespace separated chunk will be handled in a different thread.
    //
    // TODO: see what happens to the output if you insert spaces!
    let data = "86967897737416471853297327050364959
11861322575564723963297542624962850
70856234701860851907960690014725639
38397966707106094172783238747669219
52380795257888236525459303330302837
58495327135744041048897885734297812
69920216438980873548808413720956532
16278424637452589860345374828574668";
    // Make a vector to hold the child-threads which we will spawn.
    let mut children = vec![];
    /*************************************************************************
     * "Map" phase
     *
     * Divide our data into segments, and apply initial processing
     ************************************************************************/
    // split our data into segments for individual calculation
    // each chunk will be a reference (&str) into the actual data
    let chunked_data = data.split_whitespace();
    // Iterate over the data segments.
    // .enumerate() adds the current loop index to whatever is iterated
    // the resulting tuple "(index, element)" is then immediately
    // "destructured" into two variables, "i" and "data_segment" with a
    // "destructuring assignment"
    for (i, data_segment) in chunked_data.enumerate() {
        println!("data segment {} is \"{}\"", i, data_segment);
        // Process each data segment in a separate thread
        //
        // spawn() returns a handle to the new thread,
        // which we MUST keep to access the returned value
        //
        // 'move || -> u32' is syntax for a closure that:
        // * takes no arguments ('||')
        // * takes ownership of its captured variables ('move') and
        // * returns an unsigned 32-bit integer ('-> u32')
        //
        // Rust is smart enough to infer the '-> u32' from
        // the closure itself so we could have left that out.
        //
        // TODO: try removing the 'move' and see what happens
        children.push(thread::spawn(move || -> u32 {
            // Calculate the intermediate sum of this segment:
            let result = data_segment
                // iterate over the characters of our segment..
                .chars()
                // .. convert text-characters to their number value..
                .map(|c| c.to_digit(10).expect("should be a digit"))
                // .. and sum the resulting iterator of numbers
                .sum();
            // println! locks stdout, so no text-interleaving occurs
            println!("processed segment {}, result={}", i, result);
            // "return" not needed, because Rust is an "expression language", the
            // last evaluated expression in each block is automatically its value.
            result
        }));
    }
    /*************************************************************************
     * "Reduce" phase
     *
     * Collect our intermediate results, and combine them into a final result
     ************************************************************************/
    // collect each thread's intermediate results into a new Vec
    let mut intermediate_sums = vec![];
    for child in children {
        // collect each child thread's return-value
        let intermediate_sum = child.join().unwrap();
        intermediate_sums.push(intermediate_sum);
    }
    // combine all intermediate sums into a single final sum.
    //
    // we use the "turbofish" ::<> to provide sum() with a type hint.
    //
    // TODO: try without the turbofish, by instead explicitly
    // specifying the type of final_result
    let final_result = intermediate_sums.iter().sum::<u32>();
    println!("Final sum result: {}", final_result);
    //=> Final sum result: 1342
}
```


