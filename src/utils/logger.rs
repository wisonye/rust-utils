//!
//! This logger utility prints the log content based on the `LOG_LEVEL` env variable:
//!
//! `LOG_LEVEL="DEBUG"`: Print all function calls: debug_log/info_log/warn_log/error_log
//! `LOG_LEVEL="INFO"` : Only print function calls: info_log/warn_log/error_log
//! `LOG_LEVEL="WARN"` : Only print function calls: warn_log/error_log
//! `LOG_LEVEL="ERROR"`: Only print function calls: error_log
//!
//! If `LOG_LEVEL` is not provided, treat it as LOG_LEVEL="ERROR".
//!
//! Example:
//!
//! ```rust
//! use rust_utils::utils::logger;
//!
//! logger::debug_log("Main", "main", format!("p: {p:?}").as_str());
//! logger::info_log("Main", "main", format!("p: {p:?}").as_str());
//! logger::warn_log("Main", "main", format!("p: {p:?}").as_str());
//! logger::error_log("Main", "main", format!("p: {p:?}").as_str());
//! ```
//!
//! Now, you can run like this:
//!
//! ```bash
//! LOG_LEVEL="DEBUG" cargo run
//! ```
//!
//! If you want, you can turn off the ANSI colour escape output like this:
//!
//! ```bash
//! LOGGER_DISABLE_COLOR="TRUE" LOG_LEVEL="DEBUG" cargo run
//! ```
//!
use std::env;

//
// ANSI color escape contants
//
const LOG_COLOR_GREEN: &'static str = "\x1b[1;32m";
const LOG_COLOR_YELLOW: &'static str = "\x1b[1;33m";
const LOG_COLOR_RED: &'static str = "\x1b[1;31m";
const LOG_COLOR_RESET: &'static str = "\x1b[0m";

///
///
///
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum LogLevel {
    DEBUG = 1,
    INFO = 2,
    WARN = 3,
    ERROR = 4,
}

impl LogLevel {
    fn get_config_from_env() -> Self {
        let log_level_from_env_str = match env::var("LOG_LEVEL") {
            Ok(level) => level,
            Err(_) => String::from("ERROR"),
        };

        // println!(">>> log_level_from_env_str: {log_level_from_env_str}");
        match log_level_from_env_str.to_uppercase().as_str() {
            "DEBUG" => LogLevel::DEBUG,
            "INFO" => LogLevel::INFO,
            "WARN" => LogLevel::WARN,
            "ERROR" => LogLevel::ERROR,
            _ => LogLevel::ERROR,
        }
    }

    fn is_logger_disable_color() -> bool {
        match env::var("LOGGER_DISABLE_COLOR") {
            Ok(v) => v.trim().to_uppercase() == "TRUE",
            Err(_) => false,
        }
    }

    fn get_logger_prefix(self: &Self) -> &str {
        match self {
            Self::DEBUG => "(D)",
            Self::INFO => "(I)",
            Self::WARN => "(W)",
            Self::ERROR => "(E)",
        }
    }
}

///
///
///
fn log(log_level_to_check: LogLevel, module_name: &str, function_name: &str, message: &str) {
    let enable_log = LogLevel::get_config_from_env() as u8 <= log_level_to_check as u8;

    if !enable_log {
        return;
    }

    if LogLevel::is_logger_disable_color() {
        eprintln!(
            "{} [ {module_name} - {function_name} ] {message}",
            log_level_to_check.get_logger_prefix()
        );
    } else {
        println!(
            "{}{} [ {module_name} - {function_name} ] {message} {}",
            match log_level_to_check {
                LogLevel::DEBUG => "",
                LogLevel::INFO => LOG_COLOR_GREEN,
                LogLevel::WARN => LOG_COLOR_YELLOW,
                LogLevel::ERROR => LOG_COLOR_RED,
            },
            log_level_to_check.get_logger_prefix(),
            LOG_COLOR_RESET
        );
    }
}

/// Debug log
pub fn debug_log(module_name: &str, function_name: &str, message: &str) {
    log(LogLevel::DEBUG, module_name, function_name, message);
}

/// Info log
pub fn info_log(module_name: &str, function_name: &str, message: &str) {
    log(LogLevel::INFO, module_name, function_name, message);
}

/// Warning log
pub fn warn_log(module_name: &str, function_name: &str, message: &str) {
    log(LogLevel::WARN, module_name, function_name, message);
}

/// Error log
pub fn error_log(module_name: &str, function_name: &str, message: &str) {
    log(LogLevel::ERROR, module_name, function_name, message);
}
