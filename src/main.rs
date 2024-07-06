fn main () {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let y = 5;
    println!("The value of y is: {}", y);
    let y = "some string"; // previous 'y' got shadowed
    println!("The value of y is: {}", y);
    {
        println!("The value of y is: {}", y);
        let y = 100; // shadowed here too but under a scope
        println!("The value of y is: {}", y);
    }
    println!("The value of y is: {}", y);
    
    const SOME_COUNT: u32 = 100_000; 
    // const is not mutable in future, 
    // unlike a let variable that can be made mutable later.
    println!("The value of SOME_COUNT is: {}", SOME_COUNT);

}