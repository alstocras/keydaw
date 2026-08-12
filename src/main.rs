// imports
use rand::*;
use std::*;

// main function
fn main() {
    // get random num
    let rNum: u32 = rand::rng().random_range(1..=100);

    // forever loop
    loop {
        // prompt
        println!("pls guess");

        // empty string to store input
        let mut guess: String = String::new();

        // read line and give it to the mutable reference guess
        io::stdin().read_line(&mut guess).expect("fail");

        // confirm guess
        println!("you guess: {guess}");

        // scoping
        {
            // convert to string via shadowing
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            // check cases
            match guess.cmp(&rNum) {
                cmp::Ordering::Less => println!("smaller"),
                cmp::Ordering::Greater => println!("greater"),
                cmp::Ordering::Equal => {
                    println!("yay you got it!!");
                    break;
                }
            }
        }
    }
}
