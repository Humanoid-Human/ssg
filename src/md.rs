#[derive(Clone, Copy, PartialEq, Eq)]
enum Token {
    Bold, Italic, Header, Blockquote, C(char)
}

pub fn tokenize_line(s: &str) -> Vec<Token> {
    let mut out = Vec::with_capacity(s.len());

    for c in s.chars() {
        match c {
            '_' | '*' => {
                if !out.is_empty() && *out.last().unwrap() == Token::Italic {
                    out.pop();
                    out.push(Token::Bold);
                } else {
                    out.push(Token::Italic);
                }
            }, '>' => {
                if out.is_empty() || *out.last().unwrap() == Token::Blockquote {
                    out.push(Token::Blockquote);
                } else {
                    out.push(Token::C(c));
                }
            }, '#' => {
                if out.is_empty() || *out.last().unwrap() == Token::Blockquote
                    || *out.last().unwrap() == Token::Header {
                    out.push(Token::Header);
                } else {
                    out.push(Token::C(c));
                }
            },
            _ => out.push(Token::C(c)),
        }
    }

    out
}

pub fn parse_tokens(v: &Vec<Token>) -> &str {
    ""
}