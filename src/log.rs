use std::fmt::Display;

pub enum LogType {
    Info,
    // Error,
}

impl Display for LogType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogType::Info => write!(
                f,
                "{}",
                ansi_hex_color::colored("#ffffff", "#000000", "INFO")
            ),
            // LogType::Error => write!(
            //     f,
            //     "{}",
            //     ansi_hex_color::colored("#ffffff", "#ff0000", "ERROR")
            // ),
        }
    }
}

pub fn log(log_type: LogType, message: &str) {
    println!("{log_type} {message}");
}
