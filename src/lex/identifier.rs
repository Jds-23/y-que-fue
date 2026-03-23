use std::iter::Peekable;
pub fn extract_identifier(iter: &mut Peekable<impl Iterator<Item = char>>, token: &char) -> String {
    let mut identifier_vec: Vec<char> = vec![*token];
    while let Some(i) = iter.peek() {
        if i.is_alphanumeric() || *i == '_' {
            identifier_vec.push(*i);
            iter.next();
        } else {
            break;
        }
    }
    identifier_vec.iter().collect()
}
