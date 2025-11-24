use crate::utils::logger::debug_log;

const LOGGER_MODULE_NAME: &'static str = "Memory";

use std::fmt::Write;

///
/// Print the memory layout of the given variable.
///
/// Example:
///
/// ```rust
/// let temp_u8 = 0x31u8;
/// print_memory(&temp_u8, "temp_u8");
///
/// let temp_int = 0xAABBu32;
/// print_memory(&temp_int, "temp_int");
///
///
/// let temp_float = 123.13456f32;
/// print_memory(&temp_float, "temp_float");
///
///
/// struct Point { x: u16, y: u16 };
/// let p = Point { x: 0xAA, y: 0xBB };
/// print_memory(&p, "p");
///
/// struct Person {
///     first_name: &'static str,
///     last_name: &'static str,
///     age: u8
/// }
///
/// let me = Person {
///     first_name: "123",
///     last_name: "456",
///     age: 0xFF,
/// };
/// print_memory(&me, "me");
/// ```
///
/// Example output:
///
/// ```bash
/// (D) [ Memory - print_memory ] 'temp_u8', size: 1
/// (D) [ Memory - print_memory ] ------------------
/// (D) [ Memory - print_memory ] 31
/// (D) [ Memory - print_memory ] ------------------
///
/// (D) [ Memory - print_memory ] 'temp_int', size: 4
/// (D) [ Memory - print_memory ] -------------------
/// (D) [ Memory - print_memory ] BBAA0000
/// (D) [ Memory - print_memory ] -------------------
///
/// (D) [ Memory - print_memory ] 'temp_float', size: 4
/// (D) [ Memory - print_memory ] ---------------------
/// (D) [ Memory - print_memory ] E544F642
/// (D) [ Memory - print_memory ] ---------------------
///
/// (D) [ Memory - print_memory ] 'p', size: 4
/// (D) [ Memory - print_memory ] ------------
/// (D) [ Memory - print_memory ] AA00BB00
/// (D) [ Memory - print_memory ] ------------
///
/// (D) [ Memory - print_memory ] 'me', size: 40
/// (D) [ Memory - print_memory ] --------------------------------------------------------------------------------
/// (D) [ Memory - print_memory ] 4A39A3070100000003000000000000004D39A307010000000300000000000000FF00000000000000
/// (D) [ Memory - print_memory ] --------------------------------------------------------------------------------
/// ```
///
pub fn print_memory<T>(v: &T, var_name: &str) {
    const MY_LOGGER_FUNCTION_NAME: &str = "print_memory";

    let byte_size = core::mem::size_of_val(v);
    let title = format!("'{var_name}', size: {}", byte_size);

    // Use `String` as buffer (1KB init capacity)
    let mut buffer = String::with_capacity(1024);
    write!(buffer, "{title}",).unwrap();

    //
    // From the second print line, simulate the `debug_log` output format!!!
    //

    //
    // hr
    //
    let min_hr_len = title.len();
    let mut hr_len = byte_size * 2;
    if hr_len < min_hr_len {
        hr_len = min_hr_len;
    }
    write!(
        buffer,
        "\n(D) [ {LOGGER_MODULE_NAME} - {MY_LOGGER_FUNCTION_NAME} ] "
    )
    .unwrap();
    for _ in 0..hr_len {
        write!(buffer, "-").unwrap();
    }

    //
    // Print all bytes in HEX format by using a raw pointer to `u8`
    //
    let ptr = v as *const T as *const u8;

    write!(
        buffer,
        "\n(D) [ {LOGGER_MODULE_NAME} - {MY_LOGGER_FUNCTION_NAME} ] "
    )
    .unwrap();
    for index in 0..byte_size {
        unsafe {
            write!(buffer, "{:02X}", *(ptr.wrapping_add(index))).unwrap();
        }
    }

    //
    // hr
    //
    write!(
        buffer,
        "\n(D) [ {LOGGER_MODULE_NAME} - {MY_LOGGER_FUNCTION_NAME} ] "
    )
    .unwrap();
    for _ in 0..hr_len {
        write!(buffer, "-").unwrap();
    }

    // The final newline
    write!(buffer, "\n").unwrap();

    debug_log(LOGGER_MODULE_NAME, MY_LOGGER_FUNCTION_NAME, buffer.as_str())
}

///
/// Print the memory layout from the inner buffer of the given slice.
///
/// Example:
///
/// ```rust
/// #[derive(Debug, Clone)]
/// struct Point {
///     x: u16,
///     y: u16,
/// }
///
/// let mut point_list = Vec::with_capacity(2);
/// point_list.push(Point { x: 0x11, y: 0x22 });
/// point_list.push(Point { x: 0x33, y: 0x44 });
/// point_list.push(Point { x: 0x55, y: 0x66 });
/// point_list.push(Point { x: 0x77, y: 0x88 });
/// point_list.push(Point { x: 0xAABB, y: 0xCCDD });
///
/// print_memory_for_slice(&point_list[..], "point_list slice");
///
/// let temp_arr = vec![0xAAusize, 0xBB, 0xCC];
/// print_memory_for_slice(&temp_arr[..], "temp_arr slice");
///
/// //
/// // You can call `.as_bytes()` to get back a `&[u8]` from the
/// // `String` inner buffer.
/// //
/// let my_name = String::from("123456789");
/// print_memory_for_slice(my_name.as_bytes(), "my_name slice");
/// ```
///
/// Example output:
///
/// ```bash
/// (D) [ Memory - print_memory ] 'point_list slice', element byte size: 4, len: 5
/// (D) [ Memory - print_memory ] ------------------------------------------------
/// (D) [ Memory - print_memory ] 11002200|33004400|55006600|77008800|BBAADDCC
/// (D) [ Memory - print_memory ] ------------------------------------------------
///
/// (D) [ Memory - print_memory ] 'temp_arr slice', element byte size: 8, len: 3
/// (D) [ Memory - print_memory ] --------------------------------------------------
/// (D) [ Memory - print_memory ] AA00000000000000|BB00000000000000|CC00000000000000
/// (D) [ Memory - print_memory ] --------------------------------------------------
///
/// (D) [ Memory - print_memory ] 'my_name slice', element byte size: 1, len: 9
/// (D) [ Memory - print_memory ] ---------------------------------------------
/// (D) [ Memory - print_memory ] 31|32|33|34|35|36|37|38|39
/// (D) [ Memory - print_memory ] ---------------------------------------------
/// ```
pub fn print_memory_for_slice<T>(slice: &[T], var_name: &str) {
    const MY_LOGGER_FUNCTION_NAME: &str = "print_memory";

    let length = slice.len();
    let element_byte_size = core::mem::size_of::<T>();
    let total_mem_byte_size = element_byte_size * length;
    let title = format!(
        "'{var_name}', element byte size: {}, len: {}",
        element_byte_size, length
    );

    // Use `String` as buffer (1KB init capacity)
    let mut buffer = String::with_capacity(1024);
    write!(buffer, "{title}",).unwrap();

    //
    // From the second print line, simulate the `debug_log` output format!!!
    //

    //
    // hr
    //
    let min_hr_len = title.len();
    let number_of_separator = length - 1;
    let mut hr_len = total_mem_byte_size * 2 + number_of_separator;
    if hr_len < min_hr_len {
        hr_len = min_hr_len;
    }
    write!(
        buffer,
        "\n(D) [ {LOGGER_MODULE_NAME} - {MY_LOGGER_FUNCTION_NAME} ] "
    )
    .unwrap();
    for _ in 0..hr_len {
        write!(buffer, "-").unwrap();
    }

    //
    // Get back the inner buffer pointer and print all bytes in
    // HEX format by using a raw pointer to `u8`
    //
    let ptr = slice.as_ptr() as *const u8;

    write!(
        buffer,
        "\n(D) [ {LOGGER_MODULE_NAME} - {MY_LOGGER_FUNCTION_NAME} ] "
    )
    .unwrap();
    let mut wrote_bytes = 0;
    let mut draw_elements = 0;
    for index in 0..total_mem_byte_size {
        unsafe {
            write!(buffer, "{:02X}", *(ptr.wrapping_add(index))).unwrap();
        }

        //
        // Draw a separator char between element bytes
        //
        wrote_bytes += 1;
        if wrote_bytes == element_byte_size && draw_elements < length - 1 {
            write!(buffer, "|").unwrap();

            draw_elements += 1;
            wrote_bytes = 0;
        }
    }

    //
    // hr
    //
    write!(
        buffer,
        "\n(D) [ {LOGGER_MODULE_NAME} - {MY_LOGGER_FUNCTION_NAME} ] "
    )
    .unwrap();
    for _ in 0..hr_len {
        write!(buffer, "-").unwrap();
    }

    // The final newline
    write!(buffer, "\n").unwrap();

    debug_log(LOGGER_MODULE_NAME, MY_LOGGER_FUNCTION_NAME, buffer.as_str())
}
