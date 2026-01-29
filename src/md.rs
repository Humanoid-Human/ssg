#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Token {
    Bold, Italic, Header(u8), Blockquote(u8), Newline, C(char)
}

pub fn tokenize(strs: Vec<&str>) -> Vec<Token> {
    let mut out = Vec::with_capacity(strs.len() * 15);
    for s in strs {
        tokenize_line(&mut out, s);
    }

    out
}

fn tokenize_line(v: &mut Vec<Token>, s: &str) {
    let mut escape = false;
    for c in s.chars() {
        if escape {
            v.push(Token::C(c));
            escape = false;
            continue;
        }
        match c {
            '_' | '*' => if !v.is_empty() && *v.last().unwrap() == Token::Italic {
                    v.pop();
                    v.push(Token::Bold);
                } else {
                    v.push(Token::Italic);
                },
            '>' => if v.is_empty() {
                    v.push(Token::Blockquote(1));
                } else if let Token::Blockquote(x) = v.last_mut().unwrap(){
                    *x += 1;
                } else {
                    v.push(Token::C(c));
                },
            '#' => if v.is_empty() {
                    v.push(Token::Header(1));
                } else {
                    match v.last_mut().unwrap() {
                        Token::Blockquote(_) => v.push(Token::Header(1)),
                        Token::Header(x) => *x += 1,
                        _ => v.push(Token::C(c))
                    }
                },
            '\n' => v.push(Token::Newline),
            '\\' => escape = true,
            _ => v.push(Token::C(c))
        }
    }
}

pub fn parse_tokens(v: &Vec<Token>) -> &str {
    ""
}