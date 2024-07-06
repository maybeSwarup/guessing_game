fn main () {
    let x = 5;
    let y = x; // x is copied to y
    println!("x: {}, y: {}", x, y);

    let s1 = String::from("hello");
    let s2 = s1; // shadow copy, bad practice!
    let s2 = s2.clone();

    println!("{}, world!", s2);
}