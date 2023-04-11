fn main() {
    // u32 is a datatype that you need to specify else compilation error
    let guess : u32 = "42".parse().expect("Not a number!");
    println!("{guess}");

    // scalar type = single value
    // int, float, bool, char

    let eight_bit_signed : i8 = -128;
    println!("8-bit signed {eight_bit_signed}");
    let eight_bit_unsigned : u8 = 255;
    println!("8-bit unsigned {eight_bit_unsigned}");   
    let sixteen_bit_signed : i16 = -32767;
    println!("16-bit signed {sixteen_bit_signed}");
    let sixteen_bit_unsigned : u16 = 65535;   
    println!("16-bit unsigned {sixteen_bit_unsigned}");
    let thirty_two_bit_signed : i32 = -2147483647;
    println!("32-bit-signed {thirty_two_bit_signed}");
    let thirty_two_bit_unsigned : u32 = 4294967295;
    println!("32-bit-unsigned {thirty_two_bit_unsigned}");
    let sixty_four_bit_signed : i64 = -9223372036854775807;
    println!("64-bit-signed {sixty_four_bit_signed}");
    let sixty_four_bit_unsigned : u64 = 18446744073709551615;
    println!("64-bit-unsigned {sixty_four_bit_unsigned}");
    let hundred_twenty_eight_bit_signed : i128 = -170141183460469231731687303715884105728;
    println!("128-bit-signed {hundred_twenty_eight_bit_signed}");
    let hundred_twenty_eight_bit_unsigned : u128 = 340282366920938463463374607431768211455;
    println!("128-bit-signed {hundred_twenty_eight_bit_unsigned}");
    println!("arch is dependent of architecture of the computer");
    let arch_signed : isize = -9223372036854775808;
    println!("arch-signed {arch_signed}");
    let arch_unsigned : usize = 18446744073709551615;
    println!("arch-unsigned {arch_unsigned}");

    // Rust panic if overflow in debug mode and will auto resolve in release mode
    // ex: going from 256 -> 0, 257 -> 1 and so on...
    // wrapping_* = wrap in all mode
    // checked_* = return None if overflow
    // overflow_* = return value and bool indicating overflow
    // saturating_* = saturate at the value's min or max
    // ex:
    // u32::wrapping_add;
    // u32::checked_add;
    // u32::overflowing_add;
    // u32::saturating_add;
    
    // Float
    let x = 2.0; // f64 double precision
    println!("float double precision : {x}");
    let y: f32 = 3.0; // f32 single precision
    println!("float single precision : {y}");
    
    // Numeric Operations
    // addition
    let sum = 5 + 10;
    println!("5 + 10 = {sum}");
    // subtraction
    let difference = 95.5 - 4.3;
    println!("95.5 - 4.3 = {difference}");
    // multiplication
    let product = 4 * 30;
    println!("4 * 30 = {product}");
    // division
    let quotient = 56.7 / 32.2;
    println!("56.7 / 32.2 = {quotient}");
    let truncated = -5 / 3; // Results in -1
    println!("-5 / 3 = {truncated}");
    // remainder
    let remainder = 43 % 5;
    println!("43 % 5 = {remainder}");

    // Bool
    let t = true;
    println!("{t}");
    let f: bool = false; // with explicit type annotation
    println!("{f}");

    let c = 'z';
    println!("{c}");
    let z: char = 'ℤ'; // with explicit type annotation
    println!("{z}");
    let heart_eyed_cat = '😻';
    println!("{heart_eyed_cat}");

    // Compound Types
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // Pattern matching for destructuring
    let (r,g,b) = tup;
    println!("{r},{g},{b}");

    let tup2 = (500, 6.4, 1);
    let (x, y, z) = tup2;
    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("{five_hundred}, {six_point_four}, {one}");
    //wont work : println!("{x.0}, {x.1}, {x.2}");

    // Array
    // data collected on stack instead of heap
    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
    "August", "September", "October", "November", "December"];
    let a2: [i32; 5] = [1, 2, 3, 4, 5];
    let a3 = [3; 5]; // a = [3, 3, 3, 3, 3];
    println!("{:?}", a);
    println!("{:?}", a2);
    println!("{:?}", a3);
    println!("{:?}", months);
    for index in 0..a.len() {
        println!("index: {}, value: {}", index, a[index]);   
    }
    for value in a.iter() {
        println!("{}", value);    
    }
}
