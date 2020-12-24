

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

