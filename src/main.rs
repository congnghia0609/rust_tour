


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

