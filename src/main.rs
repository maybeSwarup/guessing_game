fn main () {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    
    println!("{}, {}", r1, r2);

    // unsaid rule here
    // r1, r2 automatically gone out of scope
    // when not used further

    let r3 = &mut s;

    println!("r3: {}", r3);

    // println!("{}, {}", r1, r2);
    // above line will make r1, r2
    // come inside scope again
    // then r3 cannot be a mutable reference
}