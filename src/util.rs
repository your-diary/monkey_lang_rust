pub fn is_letter(c: char) -> bool {
    c.is_ascii_alphabetic() || (c == '_')
}

pub fn typename<T>(_: &T) -> &str {
    std::any::type_name::<T>()
}
