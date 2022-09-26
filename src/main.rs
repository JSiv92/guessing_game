//import standard I/O lib
use std::io;

fn main() {
    println!("Guess the number");

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

    println!("You guessed: {guess}");
}
