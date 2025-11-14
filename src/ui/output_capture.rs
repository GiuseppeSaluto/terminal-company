use std::sync::Mutex;

lazy_static::lazy_static! {
    pub static ref MESSAGE_BUFFER: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

pub fn add_message(msg: String) {
    if let Ok(mut buffer) = MESSAGE_BUFFER.lock() {
        buffer.push(msg);
    }
}

pub fn drain_messages() -> Vec<String> {
    if let Ok(mut buffer) = MESSAGE_BUFFER.lock() {
        buffer.drain(..).collect()
    } else {
        Vec::new()
    }
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
