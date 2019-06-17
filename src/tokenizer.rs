#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    Number(i32),
    Identifier(String),
    String(String),
    OpenParen,
    CloseParen,
}

pub fn tokenizer(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = vec![];
    let mut iter = input.chars().peekable();

    loop {
        let ch = iter.next();

        if ch.is_none() {
            break;
        };

        let ch = ch.unwrap();

        match ch {
            '(' => tokens.push(Token::OpenParen),
            ')' => tokens.push(Token::CloseParen),
            ' ' => continue,

            n @ '0'...'9' => {
                let mut number = String::new();
                number.push(n);
                loop {
                    match iter.peek() {
                        Some('0'...'9') => {
                            number.push(iter.next().expect("number"));
                        }
                        _ => {
                            break;
                        }
                    }
                }
                tokens.push(Token::Number(number.parse().unwrap()));
            }

            '"' => {
                let mut name = String::new();
                loop {
                    match iter.peek() {
                        Some('"') => {
                            iter.next();
                            break;
                        }
                        _ => {
                            name.push(iter.next().expect("letter"));
                        }
                    }
                }
                tokens.push(Token::String(name));
            }

            i @ 'a'...'z' | i @ 'A'...'Z' => {
                let mut name = String::new();
                name.push(i);

                loop {
                    match iter.peek() {
                        Some('a'...'z') | Some('A'...'Z') | Some('_') => {
                            name.push(iter.next().expect("letter 2"));
                        }
                        _ => break,
                    }
                }
                tokens.push(Token::Identifier(name));
            }

            c @ _ => {
                return Err(format!("Unexpected token: {}", c));
            }
        };
    }
    Ok(tokens)
}

#[test]
fn test_tokenizer() {
    let input = "(add 2 (subtract 4 2))";
    let tokens = vec![
        Token::OpenParen,
        Token::Identifier("add".to_string()),
        Token::Number(2),
        Token::OpenParen,
        Token::Identifier("subtract".to_string()),
        Token::Number(4),
        Token::Number(2),
        Token::CloseParen,
        Token::CloseParen,
    ];
    println!("{:?}", tokenizer(&input));
    assert!(tokenizer(&input) == Ok(tokens));
}
