use rand::Rng;
use std::{
    any::{type_name},
    cmp::Ordering,
    io::{self, Write},
};

fn main() {
    loop {
        println!("Select from below, enter 0 to exit.");
        println!("1. Guessing Game");
        println!("2. Mutability");
        println!("3. Shadowing");
        println!("4. Floating Point Numbers");
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

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
            4 => floating_point_numbers(),
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

    /*
      Notice the "mut" keyword used here, it enables us to declare a variable as mutable.
      By default all variables declared in Rust with let are immutable.
    */
}

fn shadowing() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("Value of x in the inner scope is: {x}");
    }

    println!("Value of x is: {x}");

    /*
       variable x is shadowed in outer scope and inner scope.
       Inner scope shadowing ends as soon as inner scope ends.
    */
}

fn floating_point_numbers() {
    let x = 2.0;

    println!("Data type of x is {}", get_type_of(&x));

    let y: f32 = 3.0;

    println!("Data type of y is {}", get_type_of(&y));

    /*
       By Default f64 floating type is used in Rust as f64 and f32 have almost same performance and speed on modern OSs.
       With f64 we get more precision than f32.
    */
}

fn get_type_of<T>(_: &T) -> &str {
    return type_name::<T>();
}
