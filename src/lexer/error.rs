pub enum LexError {
    UnterminatedString { line: i32 },
    UnexpectedCharacter { line: i32, ch: char },
}
