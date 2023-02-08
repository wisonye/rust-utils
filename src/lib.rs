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

        use crate::utils::logger::{debug_log, LogLevel};
        use crate::utils::memory;
        use std::env;

        const MEMORY_LOGGER_NAME: &'static str = "MemoryTests";

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
            env::set_var("LOG_LEVEL", "debug");
            let log_level = LogLevel::get_config_from_env();

            let color = Color {
                red: 0xAA,
                green: 0xBB,
                blue: 0xCC,
            };

            let memory_info = memory::get_memory_block_info(&color);

            debug_log(
                log_level,
                MEMORY_LOGGER_NAME,
                "should_get_memory_block_info",
                &format!("{memory_info:#?}"),
            );

            assert_eq!(memory_info.block_size, 3);
            assert_eq!(memory_info.block_hex, "AABBCC");
        }
    }

    //
    // Hex
    //
    mod hex_tests {
        use crate::utils::hex;
        use crate::utils::logger::{debug_log, LogLevel};
        use std::env;

        const HEX_LOGGER_NAME: &'static str = "HexTests";

        #[test]
        fn hex_to_string_should_work() {
            env::set_var("LOG_LEVEL", "debug");
            let log_level = LogLevel::get_config_from_env();

            let hex_arr = vec![0xAAu8, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF];
            let hex_str = hex::hex_to_string(&hex_arr, None);

            debug_log(
                log_level,
                HEX_LOGGER_NAME,
                "hex_to_string_should_work",
                &format!("hex_str: {hex_str}"),
            );

            assert_eq!(hex_str.len(), 12);
            assert_eq!(hex_str, "AABBCCDDEEFF");

            let hex_str_with_space = hex::hex_to_string(&hex_arr, Some(' '));
            debug_log(
                log_level,
                HEX_LOGGER_NAME,
                "hex_to_string_should_work",
                &format!(">>> hex_str_with_space: '{hex_str_with_space}'"),
            );

            assert_eq!(hex_str_with_space.len(), 12 + 5);
            assert_eq!(hex_str_with_space, "AA BB CC DD EE FF");
        }
    }
}
