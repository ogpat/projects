// import the standard library for rust to use the io trait
use std::io;
// import the rand library and ran
use rand::Rng;
// import the Ordering enum from the standard library
use std::cmp::Ordering;

fn main() {
    //Welcome message
    println!("\nWelcome to the number guessing game \n");

    //generate random number
    let secret = rand::thread_rng().gen_range(1..101);

    //Prompt User
    println!("Try to guess the secret number");
    
    //print secret number
    //println!("Secret number is {}", secret);

    //variable for user input
    let mut user_input =  String::new();

    //loop creates an infinite loop 
    loop
    {
    //variable for user input
    let mut user_input =  String::new();
    //let system know to accept user input
    io::stdin().read_line(&mut user_input).expect("Failed to read line!");

    //u32 = unsigned 32 bit integer | parse into integer, parse will only know when you mention u32 
    let user_input: u32 = match user_input.trim().parse(){
        //these are considered "arms" and they are used when using the match expression
        Ok(num) => num,
        Err(_) => continue,
    };
    //let use know their guess
    println!("You guess was {}", user_input);

    //compare user input against secret number
    match user_input.cmp(&secret) {
        //these options (Equal, Greater, Less) are called "variants"
        Ordering::Greater => println!("Your guess was too big"),
        Ordering::Less => println!("Your guess was too small"),
        Ordering::Equal => {println!("Your guess was correct!");
        break;
        }
    }

    }
    //Share random number with other
    println!("The number to guess was {}", secret);

    //generate a random number / must add "rand" dependency in cargo.toml file
}
