pub fn is_letter(c: char) -> bool {
    c.is_ascii_alphabetic() || (c == '_')
}
