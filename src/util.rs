pub fn is_identifier(c: char) -> bool {
    c.is_ascii_alphabetic() || c.is_ascii_digit() || (c == '_')
}

pub fn typename<T>(_: &T) -> &str {
    std::any::type_name::<T>()
}
