// Standard library documentation:
// https://doc.rust-lang.org/std/prelude/index.html
use std::io;

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
    let secret_num = rand::thread_rng().gen_range(1..=100);

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

    // String interpolation
    println!("You guessed: {guess}");
}
