use std::iter::Peekable;

pub fn extract_number_literal(
    iter: &mut Peekable<impl Iterator<Item = char>>,
    token: &char,
) -> Option<String> {
    let mut before_decimal: Vec<char> = vec![*token];
    let mut after_decimal: Vec<char> = vec![];
    while let Some(t) = iter.peek() {
        match t {
            '0'..='9' => {
                before_decimal.push(*t);
                iter.next();
            }
            '.' => {
                iter.next();
                while let Some(t) = iter.peek() {
                    match t {
                        '0'..='9' => {
                            after_decimal.push(*t);
                            iter.next();
                        }
                        '.' => {
                            return None;
                            // eprintln!("[line {}] Error: Unexpected character.", line);
                            // std::process::exit(65);
                        }
                        _ => break,
                    }
                }
                break;
            }
            _ => {
                break;
            }
        }
    }
    let s: String = if after_decimal.is_empty() {
        before_decimal.iter().collect()
    } else {
        format!(
            "{}.{}",
            before_decimal.iter().collect::<String>(),
            after_decimal.iter().collect::<String>(),
        )
    };
    Some(s)
}
