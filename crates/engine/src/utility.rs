pub fn remove_whitespace(s: &str) -> String {
    s.replace(&[' ', '\n', '\r', '\t'][..], "")
}

pub fn arrange_string(s: &str) -> String {
    remove_whitespace(s).to_ascii_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_whitespace_() {
        assert_eq!(remove_whitespace(" a\n b\r c\t d "), "abcd")
    }

    #[test]
    fn unify_string_() {
        assert_eq!(arrange_string(" 0\n 1\r 2\t 3 A\n b\r c\t d "), "0123abcd");
    }
}
