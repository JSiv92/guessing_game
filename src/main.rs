//import standard I/O lib
use std::io;
//import random
use rand::Rng;
//ordering
use std::cmp::Ordering;

fn main() {
    println!("Guess the number");
    
    //4. Random number
    let secret_number = rand::thread_rng().gen_range(1..=100);

    //display random number for testing - delete after
    println!("The secret number is: {secret_number}");

    //1.take guess input
    println!("Please input your guess:");

    //2.an empty mutable variable of string type
    let mut guess = String::new();

    /*
    3. call readline on the 'guess' variable which is:
        mutable => mut 
        an instantiated variable => &
    */
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    //5. Convert guess to int with validation
    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {guess}");

    //5. Comparing guesses using match keyword
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too low"),
        Ordering::Greater => println!("Too high"),
        Ordering::Equal => println!("Winner"),
    }

}
