use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;
fn main() {
    //print
    println!("Guess the Number!");
    

    //Generating a randon number between 1 - 101
    let secret_number = rand::thread_rng().gen_range(1..101);

    
    loop {
        //Initializing a String
        let mut guess = String::new();
        println!("Please input your guess:");
        //Getting the user input
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        //Converting a string to an integer and also err handling
        let guess  = match guess.trim().parse::<u32>(){
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Your guess: {}", guess);

        //Comparing Two numbers
        match guess.cmp(&secret_number){
            Ordering::Equal => {
            println!("{}","You Win".green());
            break;
        },
            Ordering::Greater => println!("{}", "Too Big!".red()),
            Ordering::Less => println!("{}","Too small".red())
        }
    }
   
}