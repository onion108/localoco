//! This module contains some utility functions that you may use in your application.

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
/// A brief example of a string with placeholders:
/// 
/// "Placeholder is a {0} with something that needs to be inserted {1}. {{{{}}}} can be used when
/// you want to express {{ or }}"
/// 
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
                    '}' => {
                        state = 3;
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
                        if let Ok(idx) = str::parse::<usize>(&buffer) {
                            if idx < args.len() {
                                result.push_str(args[idx]);
                            } else {
                                result.push_str("<failed insertion due to array out-of-range>")
                            }
                        } else {
                            result.push_str("<failed insertion due to invalid number format>")
                        }
                        buffer = String::new();
                        state = 0;
                    }
                    _ => {
                        return "<invalid format>".into();
                    }
                }
            }
            3 => {
                match i {
                    '}' => {
                        buffer.push('}');
                        state = 0;
                    }
                    any => {
                        buffer.push('}');
                        buffer.push(any);
                        state = 0;
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
