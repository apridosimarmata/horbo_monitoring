use std::{error::Error, fmt::Display, vec};


#[derive(Debug)]
pub struct Err <'a>  {
    msg: &'a str
}

impl <'a> Display for Err <'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl <'a> Error for Err <'a> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}


pub(crate) fn one_byte_int_to_hex_u8(val: i8) -> Result<Vec<u8>, &'static str> {
    let mut hex_bytes: Vec<u8> = Vec::new();
    let hex_chars = b"0123456789ABCDEF"; // Byte string for hex digits
    let mut value = val as u8; // Treat as unsigned for bit manipulation

    for _ in 0..2 {
        let remainder = value % 16;
        hex_bytes.push(hex_chars[remainder as usize]);
        value /= 16;
    }

    hex_bytes.reverse();
    dbg!(hex_bytes.clone());
    Ok(hex_bytes)
}