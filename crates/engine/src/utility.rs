pub fn remove_whitespace(s: &str) -> String {
    s.replace(&[' ', '\t', '\r', '\n'][..], "")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_whitespace_() {
        assert_eq!(remove_whitespace(" a\t b\r c\n d "), "abcd")
    }
}
