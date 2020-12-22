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
