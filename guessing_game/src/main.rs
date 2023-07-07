// Standard library documentation:
// https://doc.rust-lang.org/std/prelude/index.html
use std::io;
use std::cmp::Ordering;
use std::env;
use std::process;

// Note that we use ::Rng because Rng TRAIT defines
// methods that random number generators implement.
// This trait must be in scope for us to use these
// methods.
use rand::Rng;

const MSG_ARGS_TOO_FEW: &str = "Too few command line arguments.";
const MSG_ARGS_TOO_MNY: &str = "Too many command line arguments.";

fn min_max_from_args() -> [u32; 2] {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("ERROR IN ARGS: {}", MSG_ARGS_TOO_FEW);
        process::exit(1);
    }
    if args.len() > 3 {
        println!("ERROR IN ARGS: {}", MSG_ARGS_TOO_MNY);
        process::exit(1);
    }
    return [
        args[1].trim().parse().expect("Bad text input."),
        args[2].trim().parse().expect("Bad text input.")];
}

fn main() {
    let [_min, _max] = min_max_from_args();
    println!("Guess the number!");
    // Random number from 1 to 100 inclusive
    // Note inclusivity of end guaranteed by =
    // thread_rng() gives us a random number generator local
    // to the current thread of execution, and seeded by OS.
    // After this, we call gen_range on this random number
    // generator.
    println!("Welcome to the guessing game!");
    println!("Values range from {_min} to {_max}.");
    let secret_num = rand::thread_rng().gen_range(_min..=_max);

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
            Ok(num) if (_min..=_max).contains(&num) => num,
            Ok(_)  => {
                println!("Please enter a number between {_min} and {_max}.");
                continue;
            },
            Err(_) => {
                println!("Please enter a valid radix 10 number!");
                continue;
            },
        };

        // Another example which uses a nested matching.
        // let guess = match guess.trim().parse::<u32>() {
        //     Ok(num) => match num {
        //         1..=10 => num,
        //              _ => {
        //             println!("Pick a number between 1 and 10.");
        //             continue;
        //         }
        //     },
        //     Err(_)  => {
        //         println!("Please enter a valid radix 10 number!");
        //         continue;
        //     },
        // };

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
