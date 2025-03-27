use std::io::{self};

fn main() {
    loop {
        println!("Please input years:");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Could not read input!");

        // checks if the user input 'exit' to close the program
        if input.trim() == "exit" {
            println!("Bye bye :3");
            break;
        }

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let age: u32 = input * 365;

        println!("You are around {age} days old!");
    }
}