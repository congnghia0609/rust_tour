
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

