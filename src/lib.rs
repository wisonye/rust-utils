#![feature(pointer_byte_offsets)]
mod utils;

#[cfg(test)]
mod lib_tests {

    // ///
    // /// Logger
    // ///
    // mod logger_tests {

    //     use crate::utils::logger::{info_log, LogLevel};
    //     use std::env;

    //     #[test]
    //     fn config_should_work() {
    //         env::set_var("LOG_LEVEL", "DEBUG");
    //         assert_eq!(LogLevel::get_config_from_env(), LogLevel::DEBUG);

    //         env::set_var("LOG_LEVEL", "INFO");
    //         assert_eq!(LogLevel::get_config_from_env(), LogLevel::INFO);

    //         env::set_var("LOG_LEVEL", "WARNING");
    //         assert_eq!(LogLevel::get_config_from_env(), LogLevel::WARNING);

    //         env::set_var("LOG_LEVEL", "ERROR");
    //         assert_eq!(LogLevel::get_config_from_env(), LogLevel::ERROR);

    //         env::set_var("LOG_LEVEL", "WRONG_CONFIG_VALUE");
    //         assert_eq!(LogLevel::get_config_from_env(), LogLevel::ERROR);

    //         env::set_var("LOG_LEVEL", "debug");
    //         assert_eq!(LogLevel::get_config_from_env(), LogLevel::DEBUG);

    //         env::set_var("LOG_LEVEL", "info");
    //         assert_eq!(LogLevel::get_config_from_env(), LogLevel::INFO);

    //         env::set_var("LOG_LEVEL", "warning");
    //         assert_eq!(LogLevel::get_config_from_env(), LogLevel::WARNING);

    //         env::set_var("LOG_LEVEL", "error");
    //         assert_eq!(LogLevel::get_config_from_env(), LogLevel::ERROR);

    //         env::set_var("LOG_LEVEL", "wrong_config_value");
    //         assert_eq!(LogLevel::get_config_from_env(), LogLevel::ERROR);
    //     }

    //     #[test]
    //     fn should_print_info_log() {
    //         const LOGGER_NAME: &'static str = "WebServer";

    //         env::set_var("LOG_LEVEL", "info");
    //         let log_level = LogLevel::get_config_from_env();
    //         let service_name = "My Web Server";
    //         let listen_address = "0.0.0.0";
    //         info_log(
    //             log_level,
    //             LOGGER_NAME,
    //             "main",
    //             &format!("{service_name } is listening on: {listen_address}"),
    //         );
    //     }
    // }

    ///
    /// Memory
    ///
    mod memory_tests {

        use crate::utils::memory;
        use std::env;

        #[derive(Debug)]
        struct Color {
            pub red: u8,
            pub green: u8,
            pub blue: u8,
        }

        #[test]
        fn should_print_memory_block() {
            env::set_var("LOG_LEVEL", "debug");
            let color = Color {
                red: 0xAA,
                green: 0xBB,
                blue: 0xCC,
            };
            memory::print_memory_block(&color);
        }

        #[test]
        fn should_get_memory_block_info() {
            let color = Color {
                red: 0xAA,
                green: 0xBB,
                blue: 0xCC,
            };

            let memory_info = memory::get_memory_block_info(&color);
            println!("{memory_info:#?}");

            assert_eq!(memory_info.block_size, 3);
            assert_eq!(memory_info.block_hex, "AABBCC");
        }
    }
}
