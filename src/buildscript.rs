//! This module contains utilify functions for build scripts to compile json files into binary
//! files.

use std::{fs::OpenOptions, io::Read};

use crate::compiler::compile;

/// Build the given l10n json file into bytes.
pub fn build_json(file_path: &str) -> std::io::Result<Vec<u8>> {
    let mut f = OpenOptions::new().read(true).open(file_path)?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;
    match compile(&match json::parse(&buf) {
        Ok(v) => v,
        Err(err) => return Err(std::io::Error::new(std::io::ErrorKind::Other, err)),
    }) {
        Ok(v) => Ok(v),
        Err(err) => return Err(std::io::Error::new(std::io::ErrorKind::Other, err)),
    }
}
