pub fn format_name(name: &str) -> String {
    let mut chars = name.chars();
    match chars.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str().to_lowercase().as_str(),
    }
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
