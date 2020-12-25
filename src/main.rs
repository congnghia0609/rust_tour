
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

