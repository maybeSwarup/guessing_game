/*
    Rules of References: 
    1. At any given time, you can have 
    either one mutable or many immutable references.
    2. References must always be valid
*/

fn main () {
    let reference_to_nothing = dangle();
    println!("s: {}", reference_to_nothing);
}

// below code is not compilable
// because you can't return a 
// scoped reference out of the scope
fn dangle () -> &String {
    let s = String::from("hello");
    &s
    // scope ended
}