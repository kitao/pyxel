pub fn remove_whitespace(s: &str) -> String {
    s.replace(&[' ', '\t', '\r', '\n'][..], "")
}
