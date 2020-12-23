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
