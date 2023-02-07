mod utils;

#[cfg(test)]
mod lib_tests {
    use crate::utils::logger;

    use super::*;

    ///
    /// Logger
    ///
    mod logger_tests {

        #[test]
        fn config_should_work() {
            use super::utils::logger::LogLevel;
            use std::env;

            env::set_var("LOG_LEVEL", "DEBUG");
            assert_eq!(LogLevel::get_config_from_env(), LogLevel::DEBUG);

            env::set_var("LOG_LEVEL", "INFO");
            assert_eq!(LogLevel::get_config_from_env(), LogLevel::INFO);

            env::set_var("LOG_LEVEL", "WARNING");
            assert_eq!(LogLevel::get_config_from_env(), LogLevel::WARNING);

            env::set_var("LOG_LEVEL", "ERROR");
            assert_eq!(LogLevel::get_config_from_env(), LogLevel::ERROR);

            env::set_var("LOG_LEVEL", "WRONG_CONFIG_VALUE");
            assert_eq!(LogLevel::get_config_from_env(), LogLevel::ERROR);

            env::set_var("LOG_LEVEL", "debug");
            assert_eq!(LogLevel::get_config_from_env(), LogLevel::DEBUG);

            env::set_var("LOG_LEVEL", "info");
            assert_eq!(LogLevel::get_config_from_env(), LogLevel::INFO);

            env::set_var("LOG_LEVEL", "warning");
            assert_eq!(LogLevel::get_config_from_env(), LogLevel::WARNING);

            env::set_var("LOG_LEVEL", "error");
            assert_eq!(LogLevel::get_config_from_env(), LogLevel::ERROR);

            env::set_var("LOG_LEVEL", "wrong_config_value");
            assert_eq!(LogLevel::get_config_from_env(), LogLevel::ERROR);
        }
    }

    #[test]
    fn should_print_info_log() {
        use logger::{info_log, LogLevel};
        use std::env;

        const LOGGER_NAME: &'static str = "WebServer";

        env::set_var("LOG_LEVEL", "info");
        let log_level = LogLevel::get_config_from_env();
        let service_name = "My Web Server";
        let listen_address = "0.0.0.0";
        info_log(
            log_level,
            LOGGER_NAME,
            "main",
            &format!("{service_name } is listening on: {listen_address}"),
        );
    }
}
