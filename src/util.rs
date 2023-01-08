pub fn is_identifier(c: char) -> bool {
    c.is_ascii_alphabetic() || c.is_ascii_digit() || (c == '_')
}

pub fn is_digit(c: char) -> bool {
    c.is_ascii_digit() || (c == '.')
}

//An escaped character is of the form `\n`.
//This function receives `n` and returns `\n`, for example.
pub fn parse_escaped_character(c: char) -> char {
    match c {
        '\\' => '\\',
        '\'' => '\'',
        '"' => '"',
        '0' => '\0',
        'n' => '\n',
        'r' => '\r',
        't' => '\t',
        c => c,
    }
}
