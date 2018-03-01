extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1,101);
    println!("the secret is {}", secret_number);
    loop {
        println!("guess the number");
        let mut guess = String::new();

        // expect() crashes the program if the Result of read_line is Err
        let num_bytes = io::stdin().read_line(&mut guess)
            .expect("failed to read line");

        // type annotation, necessary for parse() to work
        // use a match to handle the error (better approach than expect)
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you guessed {}, {} bytes read", guess, num_bytes);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
        }
    }
}
