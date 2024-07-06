fn main () {
    let mut counter = 0;

    println!("LOOP");
    let result = loop {
        counter += 1;

        println!("counter is {}", counter);

        if counter == 10 {
            break counter;
        }
    };

    println!("The result is: {}", result);

    // while loop
    let mut number = 3;

    println!("\nWHILE LOOP");
    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // for loop
    let a = [10, 20, 30, 40, 50];

    println!("\nFOR LOOP");
    for element in a.iter() {
        println!("The value is: {}", element);
    }

    for number in 1..4 {
        println!("{}!", number)
    }
}