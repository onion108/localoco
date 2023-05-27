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

/// Insert contents into the placeholders in the string.
pub fn insert(fmt: &str, args: &[&str]) -> String {
    let mut result = String::new();
    let mut buffer = String::new();
    let mut state = 0;

    for i in fmt.chars() {
        match state {
            0 => {
                match i {
                    '{' => {
                        state = 1;
                    }
                    _ => {
                        result.push(i);
                    }
                }
            }
            1 => {
                match i {
                    '{' => {
                        result.push('{');
                        state = 0;
                    }
                    '0'..='9' => {
                        state = 2;
                        buffer.push(i);
                    }
                    _ => {
                        state = 0;
                    }
                }
            }
            2 => {
                match i {
                    '0'..='9' => {
                        buffer.push(i);
                    }
                    '}' => {
                        result.push_str(args[str::parse::<usize>(&buffer).expect("Invalid number format")]);
                        buffer = String::new();
                        state = 0;
                    }
                    _ => {
                        panic!("Invalid format string. ");
                    }
                }
            }
            _ => {}
        }
    }

    result
}

#[allow(unused)]
mod test {
    use super::insert;


    #[test]
    fn test_insert() {
        let result = insert("The {2} fox jumps {0} the lazy {1}{20}", 
        &[
            "over", "dog", "brown", "", "", "",
            "over", "dog", "brown", "", "", "",
            "over", "dog", "brown", "", "", "",
            "over", "dog", "?", "", "", "",
            "?", "dog", "brown", "", "", "",
        ]
    );
        assert_eq!(result, "The brown fox jumps over the lazy dog?");
    }

}
