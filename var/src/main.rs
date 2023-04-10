fn main() {


    // Immutable cannot have a second value assign
    let immutable = "I'm immutable";
    //immutable = "here is the proof";
    println!("{immutable}");

    // Mutable can change value
    let mut mutable = "I'm mutable";
    println!("{mutable}");
    mutable = "here is the proof!";
    println!("{mutable}");
    
    // const are immutable
    // they can be set at any scope
    // const are set only to constant expression 
    // not the result computed value at runtime
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Here a const : {THREE_HOURS_IN_SECONDS}");

    // Shadowing
    // Declare a new variable with same name
    let x = 5;  // is shadowed
    let x = x + 1;  // be the second
    {
        let x = x * 2;
        println!("Here a shadow.\nInner scope : x = {x}");
    }

    println!("x = {x}");

    // anti virus does not like idk why
    // let spaces = "   ";
    // let spaces = spaces.len();

    // you can change type
    let str = "83";
    let str: u32 = match str.trim().parse(){
        Ok(num) => num,
        Err(_) => return,
    };
    println!("shadow : {str}");

}
