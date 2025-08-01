/*
fn main() {
    let mut x = 5;
    println!("x is {}", x);
    x = 6;
    println!("x is now {}", x);
}

fn main() {
    const THREE_HOURS_INTO_SECONDS: u32 = 60 * 60 * 3;
    print!("three hours into seconds is {}", THREE_HOURS_INTO_SECONDS);
}

fn main() {
    let x = 5;
    let x = x + 1;

    {
        let x = 50;
        let x = x * 2;
        println!("Inner scope x is {}", x);
    }

    print!("Outer scope x is {}", x);
}


fn main() {
    let mut x = 4;
    println!("before but no let : {}", x);
    x = 34;
    print!("{}", x);
}

fn main() {
    let spaces = "   ";
    let spaces = spaces.len();
    println!("total space_length is : {}", spaces);
}


fn main() {
    let mut spaces = "   ";
    spaces = spaces.len();
    println1("bugged length is : {}", spaces);
}

use std::io;
fn main() {
    println!("input your guess : ");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to readline.");
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("enter an integer");
            return;
        }
    };
    println!("guess is : {}", guess);
}
*/
fn main() {}
