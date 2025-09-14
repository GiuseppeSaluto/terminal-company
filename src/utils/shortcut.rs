use std::io::{self, Write};

pub fn read_and_normalize_input() -> Option<String> {
    print!("> ");
    io::stdout().flush().ok()?;

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        println!("⚠️ Error reading input.");
        return None;
    }

    let input = input.trim().to_lowercase();
    Some(input)
}

pub fn println_separator() {
    println!("-------------------------------------------------------------");
}