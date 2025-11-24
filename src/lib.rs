pub mod utils;

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
        fn byte_array_to_hex_string_should_work() {
            env::set_var("LOG_LEVEL", "debug");
            let log_level = LogLevel::get_config_from_env();

            let hex_arr = vec![0xAAu8, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF];
            let hex_str = hex::byte_arr_to_hex_string(&hex_arr, None);

            debug_log(
                log_level,
                HEX_LOGGER_NAME,
                "byte_array_to_hex_string_should_work",
                &format!("hex_str: {hex_str}"),
            );

            assert_eq!(hex_str.len(), 12);
            assert_eq!(hex_str, "AABBCCDDEEFF");

            let hex_str_with_space = hex::byte_arr_to_hex_string(&hex_arr, Some(' '));
            debug_log(
                log_level,
                HEX_LOGGER_NAME,
                "byte_array_to_hex_string_should_work",
                &format!(">>> hex_str_with_space: '{hex_str_with_space}'"),
            );

            assert_eq!(hex_str_with_space.len(), 12 + 5);
            assert_eq!(hex_str_with_space, "AA BB CC DD EE FF");
        }

        #[test]
        fn hex_string_to_byte_array_should_work() {
            env::set_var("LOG_LEVEL", "debug");
            let log_level = LogLevel::get_config_from_env();

            let hex_str = "0A1B2C3D4E5F";
            let result = hex::hex_string_to_byte_arr(&hex_str);

            debug_log(
                log_level,
                HEX_LOGGER_NAME,
                "hex_string_to_byte_array_should_work",
                &format!("result: {:?}", result),
            );

            assert_eq!(result.is_ok(), true);
            let byte_arr = result.unwrap();
            assert_eq!(byte_arr.len(), 6);
            assert_eq!(byte_arr[0], 0x0A);
            assert_eq!(byte_arr[1], 0x1B);
            assert_eq!(byte_arr[2], 0x2C);
            assert_eq!(byte_arr[3], 0x3D);
            assert_eq!(byte_arr[4], 0x4E);
            assert_eq!(byte_arr[5], 0x5F);

            let back_to_hex_str = hex::byte_arr_to_hex_string(&byte_arr, None);
            debug_log(
                log_level,
                HEX_LOGGER_NAME,
                "hex_string_to_byte_array_should_work",
                &format!("back_to_hex_str: {back_to_hex_str}"),
            );
            assert_eq!(back_to_hex_str, "0A1B2C3D4E5F");

            let bad_hex_str = "012";
            let bad_result = hex::hex_string_to_byte_arr(&bad_hex_str);
            assert_eq!(bad_result.is_err(), true);
            let fail_reason = bad_result.err().unwrap();
            assert_eq!(fail_reason, "\"hex_string\" length must be even numeric.");
        }
    }

    //
    // Bits
    //
    mod bits_tests {
        use crate::utils::{
            bits,
            logger::{debug_log, LogLevel},
        };
        use std::env;

        const BITS_TEST_LOGGER_NAME: &'static str = "BitsTest";

        #[test]
        fn should_print_bits() {
            env::set_var("LOG_LEVEL", "debug");
            let log_level = LogLevel::get_config_from_env();

            bits::print_bits::<u8>(&0x08u8);
            bits::print_bits::<u16>(&0xABCDu16);
            bits::print_bits::<u32>(&0x889Eu32);
            bits::print_bits::<u32>(&0xFB56889Eu32);
            bits::print_bits::<u64>(&0x1234C78AFB56889Eu64);
        }

        #[test]
        fn should_get_bits() {
            env::set_var("LOG_LEVEL", "debug");
            let log_level = LogLevel::get_config_from_env();

            let u8_bits = bits::get_bits::<u8>(&0x08u8);
            assert_eq!(u8_bits, "00001000");

            let u16_bits = bits::get_bits::<u16>(&0xABCDu16);
            assert_eq!(u16_bits, "1010101111001101");

            let u32_bits = bits::get_bits::<u32>(&0x889Eu32);
            assert_eq!(u32_bits, "00000000000000001000100010011110");

            let u32_bits_2 = bits::get_bits::<u32>(&0xFB56889Eu32);
            assert_eq!(u32_bits_2, "11111011010101101000100010011110");

            let u64_bits = bits::get_bits::<u64>(&0x1234C78AFB56889Eu64);
            assert_eq!(
                u64_bits,
                "0001001000110100110001111000101011111011010101101000100010011110"
            );
        }

        #[test]
        fn check_bit_should_work() {
            env::set_var("LOG_LEVEL", "debug");
            let log_level = LogLevel::get_config_from_env();

            let v: usize = 0xABCD;

            debug_log(
                log_level,
                BITS_TEST_LOGGER_NAME,
                "check_bit_should_work",
                &format!("0x{:04X} bits: {:016b}", v, v),
            );

            for which_bit in 1..17 {
                debug_log(
                    log_level,
                    BITS_TEST_LOGGER_NAME,
                    "check_bit_should_work",
                    &format!(
                        "bit {:2} in '0x{:02X}' is 1?: {}",
                        which_bit,
                        v,
                        bits::bit_is_1(v as usize, which_bit)
                    ),
                );
            }
        }
    }
}
