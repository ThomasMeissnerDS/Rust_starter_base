external crate;
use rand::RNG;
use std::io;
use std::cmp::Ordering; // Ordering is an Enum

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("Guess the number!");

    loop { // this starts an infinite loop
        println!("Please input your guess!");
        let mut guess = String::new(); // this string is growable
        // ::new is a function associated to the type and not just an instance of it

        // convert user input String to a number (otherwise match fails)
        let guess: u32 = guess.trim().parse() { // parse returns a Result type
            Ok(num) => num, // Result type has variants Ok & Err
            Err(_) => continue // continue tells the program to go to the next iteration of this loop
        }

        io::stdin().read_line(&mut guess) // accept user input and store as string, returns as io::Result
            .expect("Failed to read line"); // instances of io::Result have an expect method that crashes the program

        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) { // cmp compares any two values and can be called on anything that can be compared
            // cmp returns any of the 3 enum values...match compares them
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}