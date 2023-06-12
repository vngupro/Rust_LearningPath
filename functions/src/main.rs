fn main() {
    println!("Hello, world!");
    another_function(5);
    print_labeled_measurement(5, 'h');

    /*    
    Statements are instructions that perform some action and do not return a value.
    Expressions evaluate to a resultant value.
     */

    // this is a statement
    let y = 6;

    // error let is a statement and after the = you expect expressions
    //let x = (let y = 6);

    // 6 is an expression
    // 5 + 6 is an expression
    // calling a function is an expression
    // calling a macro is an expression
    // a new scope block with curly brackets is an expression

    // expression does not include semi colon at the end,
    // if you add ; it's a statement else it's an expression
    let y = { let x = 3; x + 1};
    println!("y: {y}");

    let x = five();
    println!("x: {x}");

    let a = plus_one(5);
    println!("a: {a}");
}

// function definition is a statement
fn another_function(x: i32) {
    println!("Another function");
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value} {unit_label}");
}

// function with return value
fn five() -> i32 {
    // the return value is the value of the final expression 
    // in the block of the body function
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}