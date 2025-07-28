use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=10);

    loop {
        print!("Please input your guess : ");
        io::stdout().flush().unwrap();

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid input.");
                continue;
            }
        };
        println!("You guessed : {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small of a guess!"),
            Ordering::Greater => println!("Too large of a guess!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    println!("Press enter to exit!");
    let mut exit = String::new();
    io::stdin().read_line(&mut exit).unwrap();
}
