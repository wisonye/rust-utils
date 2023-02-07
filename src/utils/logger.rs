use std::env;

///
///
///
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LogLevel {
    DEBUG = 1,
    INFO = 2,
    WARNING = 3,
    ERROR = 4,
}

impl LogLevel {
    pub fn get_config_from_env() -> Self {
        let log_level_from_env_str = match env::var("LOG_LEVEL") {
            Ok(level) => level,
            Err(_) => String::from("ERROR"),
        };

        println!(">>> log_level_from_env_str: {log_level_from_env_str}");
        match log_level_from_env_str.to_uppercase().as_str() {
            "DEBUG" => LogLevel::DEBUG,
            "INFO" => LogLevel::INFO,
            "WARNING" => LogLevel::WARNING,
            "ERROR" => LogLevel::ERROR,
            _ => LogLevel::ERROR,
        }
    }
}

///
///
///
fn log(
    log_level_from_config: LogLevel,
    log_level_to_check: LogLevel,
    module_name: &str,
    function_name: &str,
    message: &str,
) {
    let enable_log = log_level_from_config as u8 <= log_level_to_check as u8;

    if !enable_log {
        return;
    }

    println!("[ {module_name:?} > {function_name:?} ] - {message}");
}

/// Debug log
pub fn debug_log(
    log_level_from_config: LogLevel,
    module_name: &str,
    function_name: &str,
    message: &str,
) {
    log(
        log_level_from_config,
        LogLevel::DEBUG,
        module_name,
        function_name,
        message,
    );
}

/// Info log
pub fn info_log(
    log_level_from_config: LogLevel,
    module_name: &str,
    function_name: &str,
    message: &str,
) {
    log(
        log_level_from_config,
        LogLevel::INFO,
        module_name,
        function_name,
        message,
    );
}

/// Warning log
pub fn warning_log(
    log_level_from_config: LogLevel,
    module_name: &str,
    function_name: &str,
    message: &str,
) {
    log(
        log_level_from_config,
        LogLevel::WARNING,
        module_name,
        function_name,
        message,
    );
}

/// Error log
pub fn error_log(
    log_level_from_config: LogLevel,
    module_name: &str,
    function_name: &str,
    message: &str,
) {
    log(
        log_level_from_config,
        LogLevel::ERROR,
        module_name,
        function_name,
        message,
    );
}
