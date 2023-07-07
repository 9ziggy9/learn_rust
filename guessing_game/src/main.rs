// Standard library documentation:
// https://doc.rust-lang.org/std/prelude/index.html
use std::io;
use std::cmp::Ordering;

// Note that we use ::Rng because Rng TRAIT defines
// methods that random number generators implement.
// This trait must be in scope for us to use these
// methods.
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // Random number from 1 to 100 inclusive
    // Note inclusivity of end guaranteed by =
    // thread_rng() gives us a random number generator local
    // to the current thread of execution, and seeded by OS.
    // After this, we call gen_range on this random number
    // generator.
    println!("Welcome to the guessing game!");
    println!("Values range from 1 to 10.");
    let secret_num = rand::thread_rng().gen_range(1..=10);

    // Note that String type has a string comparison
    // method which we can utilize. It appears to return
    // enum type "Ordering" which has varieties: Less,
    // Greater, and Equal.
    loop {
        println!("Please input your guess.");
        // Notice how we declare variable as MUTABLE
        // as references by default in Rust are IMMUTABLE
        // String::new() is constructor of strings
        // String type is  agorwable UTF-8 encoded bit of text
        // new() in this case is an "associated function",
        // that is a function which is implemented on a type.
        let mut guess = String::new();

        // EXPERIMENT: why &mut? Notice compiler throws 
        // type error when reference is not specifically
        // labeled as mutable.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); // catch err

        // Need to change guess to u32 type, this takes
        // only radix 10. Further, it is interesting to note
        // that Rust compiler is INFERRING what type we want
        // to parse to by the left-side assignment. We can
        // also specify using ::<> syntax.
        // TODO: take values of any radix
        // let guess: u32 = guess
        //     .trim()
        //     .parse() // this is radix 10 by default
        //     .expect("Please type a valid number!");

        // Because parse returns result, we can match on
        // Ok and Err; Ok holds correct value called num
        let guess = match guess.trim().parse::<u32>() {
            Ok(num) => match num {
                1..=10 => num,
                     _ => {
                    println!("Pick a number between 1 and 10.");
                    continue;
                }
            },
            Err(_)  => {
                println!("Please enter a valid radix 10 number!");
                continue;
            },
        };

        // String interpolation
        println!("You guessed: {guess}");

        match guess.cmp(&secret_num) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                break;
            },
        }
    }
}
