fn main () {
    let sum = add_integers(11, 22);
    println!("sum: {}", sum);
}

fn add_integers (x: i32, y: i32) -> i32 {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    x + y
}