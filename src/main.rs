use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    loop {
        println!("Select from below, enter 0 to exit.");
        println!("1. Guessing Game");
        println!("2. Mutability");
        println!("3. Shadowing");
        print!("Enter your choice: ");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to readline");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => guess_game(),
            2 => mutability(),
            3 => shadowing(),
            _other => break,
        };

        println!("=============================================");
    }
}

fn guess_game() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn mutability() {
    let mut x = 5;
    println!("The value of x is {x}");

    x = 6;
    println!("The value of x is {x}");
}

fn shadowing() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("Value of x in the inner scope is: {x}");
    }

    println!("Value of x is: {x}");
}