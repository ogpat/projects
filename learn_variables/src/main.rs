//variables and data types 

fn main() {

    //variables are immutable by default in rust
    let x = 5;
    println!("This is the value of the value of x: {}", x);

    //to make a variable mutable, use the mut keyword instead
    let mut y = 5;
    println!("This is the value of y: {}", y);

    //because y is mutable, i can update the value of it and it will be reflected
    y = 10;
    println!("This is also the value of y: {}", y);

    //Although variables are immutable in rust, we need to specify immutable variables with the const keyword
    //Constants are denoted with capital letters
    const HEIGHT : u32 = 10;

    let b = 6;
    println!("This is the vairable b: {}", b);
    
    let b : &str = "six";
    println!("This is the vairable b: {}", b);

    /* Data Types */

    //how do you (in short form) create an array with 5 values with all values equalling 1?
     let short_array = [1; 5];
    //what is the differnece between statements and expressions?

    email_builder("Patrick".to_string(), "Hughes".to_string());

    fn email_builder(first_name: String, last_name: String) {
        let mut email = String::new();
        static ENDING: &str = "@gmail.com";
        email = first_name;
        email += r#"."#;
        email += &last_name;
        email += &ENDING;
        println!("{}",email);

        

    }
}
