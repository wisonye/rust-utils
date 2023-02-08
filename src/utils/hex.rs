///
/// Convert hex (byte array) into a String
///
pub fn hex_to_string(byte_arr: &[u8], splitter: Option<char>) -> String {
    let mut hex_string = String::with_capacity(byte_arr.len() * 2);
    // println!(">>> hex_string len: {}", hex_string.capacity());
    for (index, temp_byte) in byte_arr.iter().enumerate() {
        // Format each `u8` into hex string with fixed width of 2
        if splitter != None && index < byte_arr.len() - 1 {
            hex_string.push_str((format!("{:02X}{}", temp_byte, splitter.unwrap())).as_str());
        } else {
            hex_string.push_str((format!("{:02X}", temp_byte)).as_str());
        }
    }
    hex_string
}

/**
 * `char` and `u8 (byte)` are the same type, but sometimes you still need to do a type conversion explicitly like below:
 *
 * let is_same_byte = byte_to_check == ('A' as u8);
 * let is_same_byte = byte_to_check == b'A';
 */

///
/// Convert a hex String into byte array
///
pub fn string_to_hex(hex_string: &str) -> Result<Vec<u8>, String> {
    if hex_string.len() % 2 != 0 {
        return Err(String::from(
            "\"hex_string\" must have even numeric length.",
        ));
    }
    // Create a vector to store the result
    let mut byte_array: Vec<u8> = Vec::new();
    // Create a temp string with 2 char capacity
    let mut temp_hex_string = String::with_capacity(2);
    for temp_char in hex_string.chars() {
        // Put 2 char into the `temp_hex_string` and Convert the hex string into `u8`
        temp_hex_string.push(temp_char);
        if temp_hex_string.len() == 2 {
            let temp_result = u8::from_str_radix(temp_hex_string.as_str(), 16);
            if temp_result.is_err() {
                return Err(format!(
                    "\"hex_string\" has non hex value: {}",
                    temp_hex_string.as_str()
                ));
            }

            byte_array.push(temp_result.unwrap());

            // Clear the `temp_hex_string` for the next round
            temp_hex_string.clear();
        }
    }

    // Return the `Vec<u8>`
    Ok(byte_array)
}

///
/// Convert a byte array into big endian u16
///
pub fn hex_to_be_u16(byte_array: &[u8]) -> u16 {
    let temp_buffer = [byte_array[0], byte_array[1]];

    u16::from_be_bytes(temp_buffer)
}

///
/// Convert a byte array into big endian u16
///
pub fn hex_to_be_u32(byte_array: &[u8]) -> u32 {
    let temp_buffer = [byte_array[0], byte_array[1], byte_array[2], byte_array[3]];

    u32::from_be_bytes(temp_buffer)
}
