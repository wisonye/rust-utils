use crate::utils::logger::{debug_log, LogLevel};

const LOGGER_NAME: &'static str = "Memory";
///
///
///
pub fn print_memory_block<T: std::fmt::Debug>(var: &T) {
    let logger_level = LogLevel::get_config_from_env();
    let block_size = std::mem::size_of::<T>();
    let prefix = "[ {}, size: {} ]";
    let type_name = std::any::type_name::<T>();

    let mut buffer = String::with_capacity(prefix.len() + type_name.len() + 20 + block_size * 2);

    buffer.push_str(&format!(
        "{}, size: {} , block HEX: ",
        type_name, block_size
    ));

    // Points to the address of `var`
    let init_mem_ptr: *const T = var;

    // Get back HEX string for each byte
    for index in 0..block_size {
        unsafe {
            //
            // We can't use `offset(count)` in here, as the `count` is in units
            // of `T`; e.g., a count of `3` represents a pointer offset of
            // `3 * size_of::<T>()` bytes.
            //
            // We need to move byte-by-byte
            //
            let current_mem_ptr = init_mem_ptr.byte_offset(index as isize);

            buffer.push_str(&format!("{:02X}", *(current_mem_ptr as *const u8)));
        }
    }

    debug_log(
        logger_level,
        LOGGER_NAME,
        "print_memory_block",
        &format!("{buffer}"),
    );
}
