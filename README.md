# `rust-utils`

This is my personal `Rust` utilities which contains the following modules:

</br>

- **`logger`**

    Logger util, it provides the following log functions:

    - `LogLevel::get_config_from_env()`
    - `debug_log()`
    - `info_log()`
    - `warning_log()`
    - `error_log()`

    </br>

    Example:

    ```rust
    use logger::{info_log, LogLevel};
    use std::env;

    const LOGGER_NAME: &'static str = "WebServer";

    let log_level = LogLevel::get_config_from_env();
    let service_name = "My Web Server";
    let listen_address = "0.0.0.0";
    info_log(
        log_level,
        LOGGER_NAME,
        "main",
        &format!("{service_name } is listening on: {listen_address}"),
    );
    ```

    </br>


- **`memory`**

    Memory util, it provides the following functions:

    - `print_memory_block`
    - `get_memory_block_info`

    </br>

    Example:

    ```rust
    use crate::utils::memory;
    use std::env;

    #[derive(Debug)]
    struct Color {
        pub red: u8,
        pub green: u8,
        pub blue: u8,
    }

    let color = Color {
        red: 0xAA,
        green: 0xBB,
        blue: 0xCC,
    };
    memory::print_memory_block(&color);

    // [ "Memory" > "print_memory_block" ] - rust_utils::lib_tests::memory_tests::Color, size: 3 , block HEX: AABBCC


    let memory_info = memory::get_memory_block_info(&color);
    println!("{memory_info:#?}");

    // MemoryBlockInfo {
    //     type_name: "rust_utils::lib_tests::memory_tests::Color",
    //     block_size: 3,
    //     block_hex: "AABBCC",
    // }
    ```

    </br>


- **`hex`**

    Hex util, it provides the following functions:

    - `byte_arr_to_hex_string`
    - `hex_string_to_byte_arr`
    - `hex_to_be_u16`
    - `hex_to_be_u32`

    </br>

    Example:

    - Get back hex string from byte array:

        ```rust
        let hex_arr = vec![0xAAu8, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF];
        let hex_str = hex::byte_arr_to_hex_string(&hex_arr, None);
        debug_log(
            log_level,
            HEX_LOGGER_NAME,
            "byte_array_to_hex_string_should_work",
            &format!("hex_str: {hex_str}"),
        );
        // [ "HexTests" > "byte_array_to_hex_string_should_work" ] - hex_str: AABBCCDDEEFF

        let hex_str_with_space = hex::byte_arr_to_hex_string(&hex_arr, Some(' '));
        debug_log(
            log_level,
            HEX_LOGGER_NAME,
            "byte_array_to_hex_string_should_work",
            &format!(">>> hex_str_with_space: '{hex_str_with_space}'"),
        );
        // [ "HexTests" > "byte_array_to_hex_string_should_work" ] - >>> hex_str_with_space: 'AA BB CC DD EE FF'
        ```

        </br>

    - Get back byte array from hex string

        ```rust
        let hex_str = "0A1B2C3D4E5F";
        let result = hex::hex_string_to_byte_arr(&hex_str);

        assert_eq!(result.is_ok(), true);
        let byte_arr = result.unwrap();
        assert_eq!(byte_arr.len(), 6);
        assert_eq!(byte_arr[0], 0x0A);
        assert_eq!(byte_arr[1], 0x1B);
        assert_eq!(byte_arr[2], 0x2C);
        assert_eq!(byte_arr[3], 0x3D);
        assert_eq!(byte_arr[4], 0x4E);
        assert_eq!(byte_arr[5], 0x5F);
        ```

        </br>


## How to run test

```bash
cargo +nightly test -- --nocapture
```

</br>
