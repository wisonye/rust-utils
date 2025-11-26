//!
//! # This logger macros print the log content based on the following settings:
//!
//! - The `LOG_LEVEL` and `LOGGER_DISABLE_COLOR` env variables
//! - The `DISABLE_DEBUG_LOG` feature
//!
//! ## 1. The `LOG_LEVEL` and `LOGGER_DISABLE_COLOR` env variables
//!
//! `LOG_LEVEL="DEBUG"`: Print all function calls: debug_log/info_log/warn_log/error_log
//! `LOG_LEVEL="INFO"` : Only print function calls: info_log/warn_log/error_log
//! `LOG_LEVEL="WARN"` : Only print function calls: warn_log/error_log
//! `LOG_LEVEL="ERROR"`: Only print function calls: error_log
//!
//! If `LOG_LEVEL` is not provided, treat it as `LOG_LEVEL="ERROR"`.
//!
//! Example:
//!
//! ```rust
//! use rust_utils::{debug_log, info_log, warn_log, error_log};
//! use rust_utils::logger::{log, LogLevel};
//!
//! const LOGGER_PREFIX: &'static str = "TempMain";
//!
//! fn main() {
//!     debug_log!(LOGGER_PREFIX, "main", "hello from RUST:)");
//!     info_log!(LOGGER_PREFIX,  "main", "hello from RUST:)");
//!     warn_log!(LOGGER_PREFIX,  "main", "hello from RUST:)");
//!     error_log!(LOGGER_PREFIX, "main", "hello from RUST:)");
//! }
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
//! It's very useful when you run `cargo run` inside your editor.
//!
//! Example output:
//!
//! ```bash
//! (D) [ TempMain - main ] hello from RUST:)
//! (I) [ TempMain - main ] hello from RUST:)
//! (W) [ TempMain - main ] hello from RUST:)
//! (E) [ TempMain - main ] hello from RUST:)
//! ```
//!
//!
//! ## 2. The `DISABLE_DEBUG_LOG` feature
//!
//! If you want to disable (DO NOT compile) all `debug_log!` macro (e.g. in release build),
//! you can re-export `rust-utils`'s feature into your crate by adding the following settings
//! to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! rust_utils = { path = "../rust-utils" }
//!
//! [features]
//! #
//! # Re-export `rust-utils`'s feature into current crate
//! #
//! disable-debug-log = ["rust_utils/DISABLE_DEBUG_LOG"]
//! ```
//!
//! Then, run or build the project like this:
//!
//! ```bash
//! # Build and run
//! LOG_LEVEL="DEBUG" cargo run --features disable-debug-log
//!
//! # Debug build
//! cargo build --features disable-debug-log
//!
//! # Relase build
//! cargo build --release --features disable-debug-log
//! ```
//!
//! After re-compiling, all `debug_log!` expands to an empty block `{}`.
//!
use std::env;
use std::sync::OnceLock;

//
// ANSI color escape contants
//
const LOG_COLOR_GREEN: &'static str = "\x1b[1;32m";
const LOG_COLOR_YELLOW: &'static str = "\x1b[1;33m";
const LOG_COLOR_RED: &'static str = "\x1b[1;31m";
const LOG_COLOR_RESET: &'static str = "\x1b[0m";

//
// Static global log level loads from env var: `LOG_LEVEL`
//
static ENV_LOG_LEVEL: OnceLock<LogLevel> = OnceLock::new();

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum LogLevel {
    DEBUG = 1,
    INFO = 2,
    WARN = 3,
    ERROR = 4,
}

impl LogLevel {
    fn get_config_from_env() -> &'static Self {
        //
        // `OnceLock<LogLevel>.get_or_init()` guarantees that only loads once!!!
        //
        ENV_LOG_LEVEL.get_or_init(|| {
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
        })
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
pub fn log(log_level_to_check: LogLevel, module_name: &str, function_name: &str, message: &str) {
    let enable_log = *LogLevel::get_config_from_env() as u8 <= log_level_to_check as u8;

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

//
// Debug log
//
#[macro_export]
#[cfg(not(feature = "DISABLE_DEBUG_LOG"))]
macro_rules! debug_log {
    ($module_name:expr, $function_name:expr, $message:expr) => {
        log(LogLevel::DEBUG, $module_name, $function_name, $message)
    };
}

//
// Debug log: expand to nothing if `DISABLE_DEBUG_LOG` feature is disabled
//
#[macro_export]
#[cfg(feature = "DISABLE_DEBUG_LOG")]
macro_rules! debug_log {
    ($module_name:expr, $function_name:expr, $message:expr) => {};
}

/// Info log
#[macro_export]
macro_rules! info_log {
    ($module_name:expr, $function_name:expr, $message:expr) => {
        log(LogLevel::INFO, $module_name, $function_name, $message)
    };
}

/// Warning log
#[macro_export]
macro_rules! warn_log {
    ($module_name:expr, $function_name:expr, $message:expr) => {
        log(LogLevel::WARN, $module_name, $function_name, $message)
    };
}

/// Error log
#[macro_export]
macro_rules! error_log {
    ($module_name:expr, $function_name:expr, $message:expr) => {
        log(LogLevel::ERROR, $module_name, $function_name, $message)
    };
}
