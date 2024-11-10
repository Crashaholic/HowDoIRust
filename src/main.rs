use std::io; // pretty much similar to #include <library> or using <library> or import <library>
use rand::Rng;

fn main()  // main fn
{
    println!("Guess the number!");

    /*
        The gen_range method takes a range expression 
        as an argument and generates a random number in 
        the range. The kind of range expression we’re 
        using here takes the form start..=end and is 
        inclusive on the lower and upper bounds, 
        so we need to specify 1..=100 to request 
        a number between 1 and 100.
    */
    let secret = rand::thread_rng().gen_range(1..=100);

    println!("Please input your guess.");

    let mut guess = String::new(); // without mut, the value cannot be changed...? only guessing that cuz mut = mutable

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line"); 
    /* 
       The & indicates that this argument is a reference,
       which gives you a way to let multiple parts of 
       your code access one piece of data without 
       needing to copy that data into memory multiple times.  
    */
    /* 
       If you don’t call expect, the program will compile, 
       but you’ll get a warning 
    */

    // let x = 5;
    // let y = 10;
    // println!("x = {x} and y + 2 = {}", y + 2);
    // would give
    // x = 5 and y + 2 = 12
    println!("You guessed: {}", guess);

    if (guess == secret)
    {
        println!("You win!");
    }
    else if (guess < secret)
    {
        println!("Too small!");
    }
    else 
    {
        println!("Too big!");    
    }

}
