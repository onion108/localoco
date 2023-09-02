//! This module contains things about strings data, which is basically a data structure represents
//! a strings file at runtime.

use std::collections::HashMap;

use json::{JsonValue, object};

use crate::compiler::ParseError;

/// A structure representing strings data.
#[derive(Clone, Debug)]
pub struct Strings {
    lang_id: String,
    lang_name: String,
    data: HashMap<String, String>
}

impl Strings {
    
    /// Creates a new `Strings` instance from given `lang_id` and `lang_name`.
    pub fn new(lang_id: &str, lang_name: &str) -> Self {
        Self {
            lang_id: lang_id.to_string(),
            lang_name: lang_name.to_string(),
            data: HashMap::new(),
        }
    }

    fn parse(content: &[u8]) -> Result<Self, ParseError> {
        let mut indexer = 11;
        let lang_name_len = (((content[indexer] as u32) << 24) as u32 + ((content[indexer + 1] as u32) << 16) as u32 + ((content[indexer + 2] as u32) << 8) as u32 + (content[indexer + 3]) as u32) as usize;
        indexer += 4;
        let lang_name = String::from_utf8_lossy(&content[indexer..(indexer + lang_name_len)]).to_string();
        indexer += lang_name_len;
        let lang_id_len = (((content[indexer] as u32) << 24) as u32 + ((content[indexer + 1] as u32) << 16) as u32 + ((content[indexer + 2] as u32) << 8) as u32 + (content[indexer + 3]) as u32) as usize;
        indexer += 4;
        let lang_id = String::from_utf8_lossy(&content[indexer..(indexer + lang_id_len)]).to_string();
        indexer += lang_id_len;
        let mut result = Self::new(&lang_id, &lang_name);
        while indexer < content.len() {
            let key_len = (((content[indexer] as u32) << 24) as u32 + ((content[indexer + 1] as u32) << 16) as u32 + ((content[indexer + 2] as u32) << 8) as u32 + (content[indexer + 3]) as u32) as usize;
            indexer += 4;
            let key = String::from_utf8_lossy(&content[indexer..(indexer + key_len)]);
            indexer += key_len;
            let val_len = (((content[indexer] as u32) << 24) as u32 + ((content[indexer + 1] as u32) << 16) as u32 + ((content[indexer + 2] as u32) << 8) as u32 + (content[indexer + 3]) as u32) as usize;
            indexer += 4;
            let val = String::from_utf8_lossy(&content[indexer..(indexer + val_len)]);
            indexer += val_len;
            result.add_string(&key, &val);
        }
        Ok(result)
    }

    /// Loads from a byte array which contains compiled string.
    pub fn load(bytes: &[u8]) -> Result<Self, ParseError> {
        let mut indexer = 0;
        while indexer <= bytes.len() {
            if bytes[indexer] == 0xff && bytes[indexer + 1] == 0x27 && bytes[indexer + 2] == 0xff {
                let mut buf = 0;
                let mut counter = 0;
                while counter < 8 {
                    buf = (buf << 8) + bytes[indexer + counter + 3] as usize;
                    counter += 1;
                }
                return Self::parse(&bytes[indexer..(indexer+buf as usize)]);
            }
            indexer += 1;
        }
        Err(ParseError::InvalidFormat(format!("Cannot find a header in the given byte chunk. ")))
    }

    /// Get the language's id. e.g., `zh_CN`, `en_US`, `eo_EO`, etc.
    pub fn get_lang_id(&self) -> &str {
        &self.lang_id
    }

    /// Get the name of the language, in the target language. e.g., `简体中文`, `English`, `Esperanto`, etc.
    pub fn get_lang_name(&self) -> &str {
        &self.lang_name
    }

    /// Insert a new string.
    pub fn add_string(&mut self, key: &str, value: &str) {
        self.data.insert(key.to_string(), value.to_string());
    }

    /// Getting a string by the key. If the key doesn't exist, the key itself will be returned.
    pub fn translate<'a>(&'a self, key: &'a str) -> &'a str {
        return match self.data.get(key) {
            Some(value) => value,
            None => key,
        }
    }

    /// Convert itself into a json object.
    pub fn jsonize(&self) -> JsonValue {
        object! {
            "lang_name": self.lang_name.clone(),
            "lang_id": self.lang_id.clone(),
            "data": self.data.clone(),
        }
    }

}
