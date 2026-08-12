// imports
use rand::*;
use std::*;

// main function
fn main() {
    // prompt
    println!("pls guess");

    // get random num
    let rNum = rand::rng().random_range(1..=100);

    // empty string to store input
    let mut guess: String = String::new();

    // read line and give it to the mutable reference guess
    io::stdin().read_line(&mut guess).expect("fail");

    // confirm guess
    println!("you guess: {guess}");

    // scoping
    {
        // convert to string
        let guess: u32 = guess.trim().parse().expect("invalid");

        // check cases
        match guess.cmp(&rNum) {
            cmp::Ordering::Less => println!("smaller"),
            cmp::Ordering::Greater => println!("greater"),
            cmp::Ordering::Equal => println!("yay you got it!!"),
        }
    }
}
