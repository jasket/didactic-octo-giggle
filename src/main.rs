use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please guess a number.");

        let mut num = String::new();

        io::stdin()
            .read_line(&mut num)
            .expect("Failed to read line");

        let num: u32 = match num.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {num}");

        match num.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
