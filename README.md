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


    ```bash
    cargo test -- --nocapture
    ```

    </br>

