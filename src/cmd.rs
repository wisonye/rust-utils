///
/// Execute command result
///
#[derive(Debug)]
pub enum ExecuteCommandResult {
    Success {
        cmd_desc: String,
        exit_code: Option<i32>,
        output: String,
    },
    // Fail(std::io::Error),
    Fail {
        error_message: String,
    },
}

///
/// Execute a command from the given `cmd_list` and wait for the result.
///
/// Example:
///
/// ```rust
/// use rust_utils::utils::{cmd, logger};
///
/// match cmd::execute_command(vec!["ls", "-lht", "./"]) {
///     cmd::ExecuteCommandResult::Success {
///         cmd_desc,
///         exit_code,
///         output,
///     } => {
///         logger::debug_log(
///             "Main",
///             "main",
///             std::format!(
///                 concat!(
///                     "Succeeded in executing the command with the result: {{",
///                     "\n\tcmd_desc: {}",
///                     "\n\texit_code: {:?}",
///                     "\n}}, output:\n\n{}",
///                 ),
///                 cmd_desc,
///                 exit_code,
///                 output
///             )
///             .as_str(),
///         );
///     }
///     cmd::ExecuteCommandResult::Fail { error_message } => {
///         logger::error_log(
///             "Main",
///             "main",
///             std::format!("Faild to execute command with error: {error_message}").as_str(),
///         );
///     }
/// }
/// ```
///
/// Example output:
///
/// ```bash
/// >>> Succeeded in executing the command with the result {
///         cmd_desc: ls -lht ./
///         exit_code: Some(0)
/// }, output:
///
/// total 24
/// drwxr-xr-x  8 wison  staff   256B 25 Nov 10:24 src
/// -rw-r--r--  1 wison  staff   427B 25 Nov 09:24 Cargo.lock
/// drwxr-xr-x@ 5 wison  staff   160B 25 Nov 09:24 target
/// -rw-------  1 wison  staff   590B 25 Nov 09:24 Cargo.toml
/// -rw-r--r--  1 wison  staff   1.4K  4 Nov 17:03 build.rs
/// ```
///
pub fn execute_command(cmd_list: Vec<&str>) -> ExecuteCommandResult {
    if cmd_list.len() == 0 {
        return ExecuteCommandResult::Fail {
            error_message: String::from("'cmd_list' is empty"),
        };
    }

    //
    // Build command by the first element and attach the rest elements as arguments
    //
    let mut cmd = std::process::Command::new(cmd_list[0]);
    cmd.args(&cmd_list[1..]);
    // for temp_arg in &cmd_list {
    //     cmd.arg(temp_arg);
    // }

    //
    // Get back a cmd string
    //
    let cmd_desc = cmd_list.join(" ");

    //
    // Executes the command as a child process, waiting for it to finish
    // and collecting all of its output.
    //
    let result = cmd.output();
    match result {
        Ok(output) => ExecuteCommandResult::Success {
            cmd_desc: cmd_desc,
            exit_code: output.status.code(),
            output: String::from_utf8(output.stdout).unwrap_or(String::new()),
        },

        Err(e) => ExecuteCommandResult::Fail {
            error_message: e.to_string(),
        },
    }
}
