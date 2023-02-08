///
/// Convert hex (byte array) into a String
///
pub fn byte_arr_to_hex_string(byte_arr: &[u8], splitter: Option<char>) -> String {
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
///
///
fn char_to_u8(c: char) -> u8 {
    // 0~9 (48 ~ 57)
    match c as u8 {
        48..=57 => c as u8 - 48,
        65..=70 => c as u8 - 55,
        97..=102 => c as u8 - 87,
        _ => 0,
    }
}

///
/// Convert a hex String into byte array
///
pub fn hex_string_to_byte_arr(hex_string: &str) -> Result<Vec<u8>, String> {
    if hex_string.len() % 2 != 0 {
        return Err(String::from("\"hex_string\" length must be even numeric."));
    }

    // Create a vector to store the result
    let mut byte_array: Vec<u8> = Vec::with_capacity(hex_string.len() / 2);

    // Create a temp array to hold every 2 chars
    let mut temp_hex_chars: [char; 2] = ['0'; 2];
    let mut push_index = 0;

    for temp_char in hex_string.chars() {
        temp_hex_chars[push_index] = temp_char;
        push_index += 1;

        if push_index >= 2 {
            let tens_digit = char_to_u8(temp_hex_chars[0]);
            let digit = char_to_u8(temp_hex_chars[1]);

            println!("\n>>> tens_digit: {tens_digit}, digit: {digit}");

            let temp_u8 = tens_digit * 16 + digit;
            byte_array.push(temp_u8);

            // Reset `push_index` for the next round
            push_index = 0;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_to_u8() {
        assert_eq!(char_to_u8('0'), 0);
        assert_eq!(char_to_u8('1'), 1);
        assert_eq!(char_to_u8('2'), 2);
        assert_eq!(char_to_u8('3'), 3);
        assert_eq!(char_to_u8('4'), 4);
        assert_eq!(char_to_u8('5'), 5);
        assert_eq!(char_to_u8('6'), 6);
        assert_eq!(char_to_u8('7'), 7);
        assert_eq!(char_to_u8('8'), 8);
        assert_eq!(char_to_u8('9'), 9);

        assert_eq!(char_to_u8('a'), 10);
        assert_eq!(char_to_u8('b'), 11);
        assert_eq!(char_to_u8('c'), 12);
        assert_eq!(char_to_u8('d'), 13);
        assert_eq!(char_to_u8('e'), 14);
        assert_eq!(char_to_u8('f'), 15);

        assert_eq!(char_to_u8('A'), 10);
        assert_eq!(char_to_u8('B'), 11);
        assert_eq!(char_to_u8('C'), 12);
        assert_eq!(char_to_u8('D'), 13);
        assert_eq!(char_to_u8('E'), 14);
        assert_eq!(char_to_u8('F'), 15);

        assert_eq!(char_to_u8('g'), 0);
        assert_eq!(char_to_u8('G'), 0);
    }
}
