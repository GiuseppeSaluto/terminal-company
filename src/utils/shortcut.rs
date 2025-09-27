use std::io::{self, Write};

pub fn read_and_normalize_input() -> Option<String> {
    print!("> ");
    io::stdout().flush().ok()?;

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        println!("⚠️ Error reading input.");
        return None;
    }

    let input = input.trim().to_uppercase();
    Some(input)
}

pub fn format_name(name: &str) -> String {
    let mut chars = name.chars();
    match chars.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str().to_lowercase().as_str(),
    }
}

pub fn println_separator() {
    println!("-------------------------------------------------------------");
}

#[macro_export]  // for global use
macro_rules! derive_struct {
    (
        $(#[$meta:meta])*
        pub struct $name:ident {
            $($field:tt)*
        }
    ) => {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        $(#[$meta])*
        pub struct $name {
            $($field)*
        }
    };
}
