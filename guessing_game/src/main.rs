use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Secret number: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too high"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
/*  NOTE :

   let = creating a var (by default it's not mutable = cannot change the value)
   mut = is mutable = can be change.
   read_line return Result of value Ok or Err
   Result has expect() method. If value err -> expect is call

   Shadowing not so good
   but it can kill the mutability
   avoid variable shadowing especially for local and global var
   Shadow are valid only in he scope they are declare
   Seems like it stayed because initially supported and never removed
   https://stackoverflow.com/questions/59860476/what-is-the-rationale-behind-allowing-variable-shadowing-in-ru
   let secret_word = String::new();

   match guess.cmp(&secret_word)
   {
       Ordering::Equal=> println!("You win!")
   }

   placeholders
   two type
   -> {variable}
   -> {}, value
   let x = 2;
   let y = 10;
   println!("x = {x}, y + 5 = {}", y + 5);

   parse() return a Result Ok or Err
*/
