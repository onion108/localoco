pub mod strings;
pub mod compiler;

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
