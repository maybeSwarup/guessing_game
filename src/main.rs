fn main () {
    let x = 5;
    let y = x; // x is copied to y
    println!("x: {}, y: {}", x, y);

    let s1 = String::from("hello");
    let s2 = s1; // shadow copy, bad practice!
    let s2 = s2.clone();

    println!("{}, world!", s2);

    ownership_example_1();
    ownership_example_2();
}

fn ownership_example_1 () {
    println!("\n\nOwnership Example 1\n");
    let s = String::from("hello");
    takes_ownership(s);
    // println!("s: {}", s); // Error, because ownership has been taken!

    let x = 5;
    makes_copy(x);
    println!("x: {}", x);
}

fn takes_ownership (some_string: String) {
    println!("some_string: {}", some_string);
}

fn makes_copy (some_integer: i32) {
    println!("some_integer: {}", some_integer);
}

fn ownership_example_2 () {
    println!("\n\nOwnership Example 2\n");
    let s1 = gives_ownership();
    println!("s1 = {}", s1);
    let s2 = String::from("hello");
    let s2 = takes_and_gives_back_ownership(s2);
    println!("s2 = {}", s2);
}

fn gives_ownership () -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back_ownership (a_string: String) -> String {
    a_string
}