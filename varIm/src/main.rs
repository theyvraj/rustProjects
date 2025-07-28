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
*/

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
