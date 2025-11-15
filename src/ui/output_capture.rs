use std::sync::Mutex;

lazy_static::lazy_static! {
    pub static ref MESSAGE_BUFFER: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

#[macro_export]
macro_rules! ui_println {
    () => {
        $crate::ui::output_capture::add_message(String::new())
    };
    ($($arg:tt)*) => {
        $crate::ui::output_capture::add_message(format!($($arg)*))
    };
}

#[macro_export]
macro_rules! ui_eprintln {
    () => {
        $crate::ui::output_capture::add_message(String::new())
    };
    ($($arg:tt)*) => {
        $crate::ui::output_capture::add_message(format!("⚠️ {}", format!($($arg)*)))
    };
}
