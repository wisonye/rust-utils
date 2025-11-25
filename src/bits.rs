use crate::debug_log;

#[cfg(not(feature = "DISABLE_DEBUG_LOG"))]
use crate::logger::{LogLevel, log};

use std::fmt::{Binary, UpperHex};

#[cfg(not(feature = "DISABLE_DEBUG_LOG"))]
const LOGGER_NAME: &'static str = "Bits";

///
///
///
pub fn print_bits<T: Binary + UpperHex>(v: &T) {
    let type_name = std::any::type_name::<T>();

    if type_name == "u8" {
        debug_log!(
            LOGGER_NAME,
            "print_bits",
            &format!("0x{:02X} bits: {:08b}", v, v)
        );
    } else if type_name == "u16" {
        debug_log!(
            LOGGER_NAME,
            "print_bits",
            &format!("0x{:04X} bits: {:016b}", v, v)
        );
    } else if type_name == "u32" {
        debug_log!(
            LOGGER_NAME,
            "print_bits",
            &format!("0x{:08X} bits: {:032b}", v, v)
        );
    } else if type_name == "u64" || type_name == "usize" {
        debug_log!(
            LOGGER_NAME,
            "print_bits",
            &format!("0x{:016X} bits: {:064b}", v, v)
        );
    } else {
        let _ = v;
    }
}

///
///
///
pub fn get_bits<T: Binary + UpperHex>(v: &T) -> String {
    let type_name = std::any::type_name::<T>();

    if type_name == "u8" {
        let mut bits = String::with_capacity(8);
        bits.push_str(&format!("{:08b}", v));
        return bits;
    } else if type_name == "u16" {
        let mut bits = String::with_capacity(16);
        bits.push_str(&format!("{:016b}", v));
        return bits;
    } else if type_name == "u32" {
        let mut bits = String::with_capacity(16);
        bits.push_str(&format!("{:032b}", v));
        return bits;
    } else if type_name == "u64" {
        let mut bits = String::with_capacity(16);
        bits.push_str(&format!("{:064b}", v));
        return bits;
    } else {
        return String::new();
    }
}

///
///
///
pub fn bit_is_1(v: usize, which_bit: usize) -> bool {
    (v >> (which_bit - 1)) & 0x01 == 0x01
}
