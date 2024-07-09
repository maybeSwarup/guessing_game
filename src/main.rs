fn main () {
    let mut s1 = String::from("hello");
    change(&mut s1);
    println!("s1: {}", s1); // prints "hello, world"
}

fn change (some_string: &mut String) {
    some_string.push_str(", world")
}