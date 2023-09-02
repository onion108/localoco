//! This module contains a function that compiles strings files in JSON into their binary
//! representation for faster loading at runtime.

use std::fmt::Display;

use json::JsonValue;

/// The error while compiling/decompiling.
#[derive(Debug, Clone)]
pub enum ParseError {
    MissingKey(String),
    InvalidFormat(String),
    Other(String),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::MissingKey(x) => x,
            Self::InvalidFormat(x) => x,
            Self::Other(x) => x,
        })
    }
}

fn break_down(v: u32) -> (u8, u8, u8, u8) {
    ((v >> (8 * 3)) as u8, ((v & 0x00FF0000) >> (8 * 2)) as u8, ((v & 0x0000FF00) >> 8) as u8, (v & 0x000000FF) as u8)
}

fn put_str(dist: &mut Vec<u8>, str: &str) {
    let bytes = str.to_owned().into_bytes();
    let length = break_down(bytes.len() as u32);
    dist.push(length.0);
    dist.push(length.1);
    dist.push(length.2);
    dist.push(length.3);
    for i in bytes {
        dist.push(i);
    }
}

impl std::error::Error for ParseError {}

/// Compiles give json object into bytes.
pub fn compile(obj: &JsonValue) -> Result<Vec<u8>, ParseError> {
    let mut result = Vec::<u8>::new();

    if !obj.is_object() {
        return Err(ParseError::InvalidFormat(format!("THe object must be a compound value. ")))
    }

    if !(obj.has_key("lang_name") && obj.has_key("lang_id") && obj["lang_name"].is_string() && obj["lang_id"].is_string()) {
        return Err(ParseError::InvalidFormat(format!("Error occurred while parsing the json object: Invalid format, missing key `lang_name` and `lang_id`, or their values are not strings.")))
    }
    if !(obj.has_key("data") && obj["data"].is_object()) {
        return Err(ParseError::InvalidFormat(format!("Error occurred while parsing the json object: Invalid format, missing key `data` or its value is not an object.")));
    }
    let lang_name = obj["lang_name"].to_string().into_bytes();
    let lang_id = obj["lang_id"].to_string().into_bytes();
    let lang_name_length_bytes = break_down(lang_name.len() as u32);
    let lang_id_length_bytes = break_down(lang_id.len() as u32);
    
    // Header bytes.
    result.push(0xFF);
    result.push(0x27);
    result.push(0xFF);
    // 3-10 8 bytes will be used to represent the length of the whole data.
    result.push(0x00);
    result.push(0x00);
    result.push(0x00);
    result.push(0x00);
    result.push(0x00);
    result.push(0x00);
    result.push(0x00);
    result.push(0x00);
    
    result.push(lang_name_length_bytes.0);
    result.push(lang_name_length_bytes.1);
    result.push(lang_name_length_bytes.2);
    result.push(lang_name_length_bytes.3);

    for i in lang_name {
        result.push(i);
    }

    result.push(lang_id_length_bytes.0);
    result.push(lang_id_length_bytes.1);
    result.push(lang_id_length_bytes.2);
    result.push(lang_id_length_bytes.3);

    for i in lang_id {
        result.push(i);
    }

    // Store entries.
    for i in obj["data"].entries() {
        put_str(&mut result, i.0);
        if i.1.is_string() {
            put_str(&mut result, i.1.to_string().as_str());
        } else {
            return Err(ParseError::InvalidFormat(format!("Error occurred while parsing the json object: the value of key `{}` isn't a string. ", i.0)));
        }
    }

    let total_length = result.len() as u64;
    result[3] = (total_length >> (8 * 7)) as u8;
    result[4] = ((total_length & 0x00FF0000_00000000) >> (8 * 6)) as u8;
    result[5] = ((total_length & 0x0000FF00_00000000) >> (8 * 5)) as u8;
    result[6] = ((total_length & 0x000000FF_00000000) >> (8 * 4)) as u8;
    result[7] = ((total_length & 0x00000000_FF000000) >> (8 * 3)) as u8;
    result[8] = ((total_length & 0x00000000_00FF0000) >> (8 * 2)) as u8;
    result[9] = ((total_length & 0x00000000_0000FF00) >> (8 * 1)) as u8;
    result[10] = ((total_length & 0x00000000_000000FF) >> (8 * 0)) as u8;

    Ok(result)
}
