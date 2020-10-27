use std::io;
use std::process;

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn read_input(message: &str) -> String {
    let stdin = io::stdin();
    let input = &mut String::new();
    input.clear();

    println!("{} (enter \"q\" to exit game): ", message);

    stdin.read_line(input).expect("Could not read input.");
    let trimmed = input.trim();
    if trimmed == "q" {
        process::exit(0);
    }

    return trimmed.into();
}

pub fn read_column_input() -> u8 {
    loop {
        match (&read_input("Enter column")).parse::<u8>() {
            Ok(n) => {
                if n < 1 || n > super::COLS {
                    println!("Enter a number from 1 up to {} only.", super::COLS);
                    continue;
                } else {
                    return n - 1;
                }
            }
            Err(_) => {
                println!("Only enter single digits!");
                continue;
            }
        }
    }
}
