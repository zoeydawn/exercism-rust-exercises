/// various log levels
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Debug,
}

/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    let log_level = match level {
        LogLevel::Info => "[INFO]: ",
        LogLevel::Warning => "[WARNING]: ",
        LogLevel::Error => "[ERROR]: ",
        LogLevel::Debug => "[DEBUG]: ",
    };

    return log_level.to_owned() + message;
}

pub fn info(message: &str) -> String {
    return log(LogLevel::Info, message);
}

pub fn warn(message: &str) -> String {
    return log(LogLevel::Warning, message);
}

pub fn error(message: &str) -> String {
    return log(LogLevel::Error, message);
}
