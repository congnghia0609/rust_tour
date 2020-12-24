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

