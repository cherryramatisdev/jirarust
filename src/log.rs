use serde_json::Error as SerdeError;
use std::fmt::Display;

pub enum LogType {
    Info,
    Error,
}

pub enum Message<'a> {
    Error(Box<dyn std::error::Error>),
    SerdeError(&'a SerdeError),
    String(String),
}

impl Display for LogType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogType::Info => write!(
                f,
                "{}",
                ansi_hex_color::colored("#ffffff", "#000000", "INFO")
            ),
            LogType::Error => write!(
                f,
                "{}",
                ansi_hex_color::colored("#ffffff", "#ff0000", "ERROR")
            ),
        }
    }
}

pub fn log(log_type: LogType, message: Message) {
    match &message {
        Message::Error(error) => {
            println!("{log_type} {}", error.to_string());
        }
        Message::SerdeError(error) => {
            println!("{log_type} {}", error.to_string());
        }
        Message::String(string) => {
            println!("{log_type} {}", string);
        }
    }
}
