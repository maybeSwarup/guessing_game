fn main () {
    let mut counter = 0;
    let result = loop {
        counter += 1;

        println!("counter is {}", counter);

        if counter == 10 {
            break counter;
        }
    };

    println!("The result is: {}", result);
}