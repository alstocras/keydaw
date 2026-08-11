/* // imports!!!

// main function (like c++ i think)
fn main() {
    // printer
    println!("Hello, world!");
} */

use std::io;

fn main() {
    println!("pls guess");
    let mut guess: String = String::new();
    io::stdin().read_line(&mut guess).expect("fail");
    println!("you guess: {guess}")
}
