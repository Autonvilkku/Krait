use crate::Engine;
use std::panic::Location;

/// # LogLevel
/// This enum is used to determine the color and priority of the log message.
/// ## ⚠️**Engine shouldn't be used by the user! Addons can use it but sparingly.**
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Debug,
    Engine,
}

pub struct Log {
    message: String,
    level: LogLevel,
}

/// # log_message
/// This function logs a message to the console.
/// the message printed to the console is this:
/// "[File: <file name>, Line: <line number>, Column: <column number>] <message>"
/// Log level is used to define the priority and the color of the message.
/// color is not yet available!
#[track_caller]
pub fn log_message(engine: &mut Engine, message: &str, log_level: LogLevel) {
    let location = Location::caller();
    println!(
        "[File: {}, Line: {}, Column: {}] {}",
        location.file(),
        location.line(),
        location.column(),
        message
    );
    engine.logs.push(Log {
        message: message.to_string(),
        level: LogLevel::Info,
    });
}
