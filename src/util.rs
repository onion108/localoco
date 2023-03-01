use std::{path::{Path, PathBuf}, io::Read};

use crate::strings::Strings;

/// Loads a strings file from a given path.
/// It will loads the file called `{name}.loc` under the given path.
pub fn load_strings<P: AsRef<Path>>(path: P, name: &str) -> Result<Strings, std::io::Error> {
    let mut pathbuf = PathBuf::from(path.as_ref());
    pathbuf.push(&format!("{}.loc", name));
    let mut f = std::fs::OpenOptions::new().read(true).open(pathbuf)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;
    match Strings::load(&buffer) {
        Ok(obj) => Ok(obj),
        Err(err) => Err(std::io::Error::new(std::io::ErrorKind::Other, err))
    }
}