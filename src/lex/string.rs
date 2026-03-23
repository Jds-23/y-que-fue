use std::iter::Peekable;

pub fn extract_string_literal(
    iter: &mut Peekable<impl Iterator<Item = char>>,
    line: &mut i32,
) -> Result<String, i32> {
    let mut string_literal: Vec<char> = vec![];
    let mut terminated = false;
    while let Some(t) = iter.next() {
        if t == '\n' {
            *line += 1;
        }
        if t == '\"' {
            terminated = true;
            break;
        }
        string_literal.push(t);
    }
    if !terminated {
        Err(*line)
    } else {
        Ok(string_literal.iter().collect())
    }
}
