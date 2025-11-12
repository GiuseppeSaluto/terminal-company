use terminal_company::utils::shortcut::{format_name, println_separator};

#[test]
fn test_format_name_lowercase() {
    assert_eq!(format_name("hello"), "Hello");
    assert_eq!(format_name("world"), "World");
}

#[test]
fn test_format_name_uppercase() {
    assert_eq!(format_name("HELLO"), "Hello");
    assert_eq!(format_name("WORLD"), "World");
}

#[test]
fn test_format_name_mixed_case() {
    assert_eq!(format_name("HeLLo"), "Hello");
    assert_eq!(format_name("WoRlD"), "World");
}

#[test]
fn test_format_name_single_char() {
    assert_eq!(format_name("a"), "A");
    assert_eq!(format_name("Z"), "Z");
}

#[test]
fn test_format_name_empty_string() {
    assert_eq!(format_name(""), "");
}

#[test]
fn test_format_name_multi_word() {
    assert_eq!(format_name("HELLO WORLD"), "Hello world");
    assert_eq!(format_name("new york"), "New york");
}

#[test]
fn test_format_name_with_numbers() {
    assert_eq!(format_name("test123"), "Test123");
    assert_eq!(format_name("123TEST"), "123test");
}

#[test]
fn test_format_name_special_chars() {
    assert_eq!(format_name("hello-world"), "Hello-world");
    assert_eq!(format_name("test_name"), "Test_name");
}

#[test]
fn test_format_name_whitespace() {
    assert_eq!(format_name("  hello  "), "  hello  ");
    assert_eq!(format_name(" test"), " test");
}

#[test]
fn test_format_name_unicode() {
    assert_eq!(format_name("ñoño"), "Ñoño");
    assert_eq!(format_name("über"), "Über");
}

#[test]
fn test_println_separator_does_not_panic() {
    println_separator();
}

#[test]
fn test_format_name_moon_names() {
    // Test with actual moon names from the game
    assert_eq!(format_name("EXPERIMENTATION"), "Experimentation");
    assert_eq!(format_name("VOW"), "Vow");
    assert_eq!(format_name("ASSURANCE"), "Assurance");
    assert_eq!(format_name("COMPANY"), "Company");
}

#[test]
fn test_format_name_repeated_chars() {
    assert_eq!(format_name("aaaa"), "Aaaa");
    assert_eq!(format_name("ZZZZ"), "Zzzz");
}
