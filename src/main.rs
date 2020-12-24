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

