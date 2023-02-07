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



## How to run test

```bash
cargo +nightly test -- --nocapture
```

</br>
