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
