// bring Rng trait from rand crate into scope
use rand::Rng;
// bring io from standard library into scope
use std::io;
// bring std::cmp::Ordering into scope from std library
// Ordering is an enum (Less, Greater, Equal)
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    // rand::thread_rng() particular random number generator local to current thread
    // gen_range - defined by Rng trait
    // gen_range - inclusive on lower bound, exclusive on upper bound
    let secret_number = rand::thread_rng().gen_range(1, 101);
    // loop forever
    loop {
        println!("Please input your guess.");
        // let creates an immutable variable, mut creates a mutable variable
        // String is a growable UTF-8 encoded bit of text, provided by the standard library
        // ::new - new is an associated function of the String type (static method)
        let mut guess = String::new();
        //io::stdin == std::io::stdin
        //&mut guess = reference
        // references are immutable by default
        // expect handles io::Result
        // io::Result is an enum, either Ok or Err
        // if result is Err, show error message
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // rust allows shadowing a previous variable with a new one
        // shadowing used for converting types, allows keeping same variable name
        // trim removes spaces and newlines
        // parse parses a string into a number
        // guess: u32 specifies type of number we want
        // can use match instead of expect for error handling
        let guess: u32 = match guess.trim().parse() {
            // If result is OK return number in result
            Ok(num) => num,
            // _ is a catchall
            // we want to match all errors regardless of what they contain
            // continue skips current loop execution and goes on to the next one
            Err(_) => continue,
        };
        // {} = string interpolation
        // can use multiple, ("x = {} and y = {}", x, y)
        println!("You guessed: {}", guess);

        // cmp compares two values, takes a reference to something to compare with
        // cmp returns a value from Ordering enum
        // match decides what to do based on Ordering value
        // match is kind of like a switch statement
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                // exit loop
                break;
            }
        }
    }
}
