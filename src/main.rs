extern crate rand;

use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    println!("Guess the number");
    println!("Plese input your guess");

    let secret_number = rand::thread_rng().gen_range(1, 101);



    let mut guess = String::new();



    println!("the secret number is: {}", secret_number);

    loop {
        io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number");

        println!("You guessed {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    
    }


}

