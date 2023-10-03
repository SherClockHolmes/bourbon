pub enum Token {
    FunctionCall(String, Vec<u8>),
    Placeholder(Vec<u8>),
    Text(Vec<u8>),
}

pub fn tokenize(template: &[u8]) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut i = 0;
    while i < template.len() {
        if template[i] == b'{' {
            let mut j = i + 1;
            while j < template.len() && template[j] != b'}' {
                j += 1;
            }
            if j < template.len() {
                let segment = &template[i + 1..j];
                if segment.contains(&b'(') && segment.contains(&b')') {
                    // This is a function call
                    let start_fn = segment.iter().position(|&b| b == b'(').unwrap();
                    let end_fn = segment.iter().position(|&b| b == b')').unwrap();
                    let function_name = String::from_utf8(segment[..start_fn].to_vec()).unwrap();
                    let argument = segment[start_fn + 1..end_fn].to_vec();
                    tokens.push(Token::FunctionCall(function_name, argument));
                } else {
                    tokens.push(Token::Placeholder(segment.to_vec()));
                }
                i = j + 1;
            } else {
                return Err("Unmatched {".to_string());
            }
        } else {
            let mut j = i;
            while j < template.len() && template[j] != b'{' {
                j += 1;
            }
            tokens.push(Token::Text(template[i..j].to_vec()));
            i = j;
        }
    }
    Ok(tokens)
}
