
fn main () {
    // Scaler datatypes:
    // Integers
        let a = 98_222; // Decimal
        let b = 0xff; // Hex
        let c = 0o77; // Octal
        let d = 0b1111_0000; // Binary
        let e = b'A'; // Byte (u8 only) 
    
        let f: u8 = 255 + 1; 
        // by chance if the value gets overflowed
        // it will get wrapped around to the initial value
        // like 256 will become 0 and 257 will be 1 and so forth.
        println!("Value of f: {}", f); 

    // Floating-point numbers
        let f = 2.0;
        let g: f32 = 3.0;

        // addition
        let sum = 5 + 10;
        // subtraction
        let difference = 95.5 - 4.3;
        // multiplication
        let product = 4 * 30;
        // division
        let quotient = 56.7 / 32.2;
        // ramainder
        let remainder = 43 % 5;

    // Boolean
        let t = true;
        let f: bool = false;

    // Character
        let c = 'z';
        let om = 'ૐ';
        let heart_eyed_cat = '😻';

}