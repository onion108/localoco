//! # LocaLoco
//! Localoco is a simple library that brings the ability of translating strings to your
//! application. The core of it is to use a **translation key** to replace the string to be
//! displayed and then load a specified **strings file** (according to the language/region
//! settings).
//!
//! To ensure the loading of the so-called "string file" is fast, we compile it into a binary form
//! so that it will be easier to parse at runtime. This work could be done as your Rust code being
//! compiled. The strings file will be originally written in json, which is a human-readable text
//! format, and finally transpiled into a binary form, being ready to be loaded at runtime.

pub mod strings;
pub mod compiler;
pub mod util;

/// This module contains some utilities for buildscript.
pub mod buildscript;

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn strings() {
        let mut s = strings::Strings::new("en_US", "English (US)");
        s.add_string("ui.hello", "Hello, World! ");
        s.add_string("ui.cancel", "Cancel");
        assert_eq!(s.get_lang_id(), "en_US");
        assert_eq!(s.get_lang_name(), "English (US)");
        assert_eq!(s.translate("ui.hello"), "Hello, World! ");
        assert_eq!(s.translate("key.not.exist"), "key.not.exist");
        let bytes = compiler::compile(&s.jsonize()).unwrap();
        let recov = strings::Strings::load(&bytes).unwrap();
        assert_eq!(s.translate("ui.hello"), recov.translate("ui.hello"));
    }
}
