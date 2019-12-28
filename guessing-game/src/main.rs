use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("guess a number");
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("some error happened");
        
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Less than secret number"),
            Ordering::Greater => println!("Greater than secret number"),
            Ordering::Equal => {
                println!("You won!");
                break;
            }
        }
    }
}
