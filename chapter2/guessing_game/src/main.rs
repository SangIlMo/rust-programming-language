use rand::Rng; // trait that defines random number generators implementations
use std::cmp::Ordering;
use std::io; // tool for accepting user input

fn main() {
    println!("Guess the number!");

    // using form start..=end
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess");

        // let - to declare variable
        // mut - convert variable to mutable from immutable (by default)
        // String:new() - return of new instance of String
        // new function on many types in general
        let mut guess = String::new(); // new instance of String is bound to mutable variable "guess"

        // from std::io
        // call stdin() from io
        io::stdin() // new instance of std::io::Stdin (handle to the standard input)
            // append user input into guess variable
            // guess should be mutable to append user input
            // & - means reference of variable, references are immutable
            // you need to write &mut to make mutable
            .read_line(&mut guess) // returns Result value (enumeration)
            // Result - enumeration for error handling, compose of 'Ok' and 'Err'
            // expect() - when 'Ok' return the value, or 'Err' error from underlying OS
            // if you not use expect(), it will be compiled but warns
            .expect("Failed to read line");

        // shadowing - reuse variable name
        // convert a value from one type to another type.
        let guess: u32 = match guess.trim().parse() { // match each case Ok, Err
            Ok(num) => num, // if Ok, return num
            Err(_) => continue, // if Err(_), _ is all error case, continue loop
        };

        // {} bracket - print the value of variable inside the bracket
        // if empty bracket, it will print the variable provided after 'comma'
        // ex, println!("x = {}", x); it will print x's value
        println!("You guessed: {guess}");

        // call each case is arm
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
