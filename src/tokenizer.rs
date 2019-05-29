
pub struct Token {
   pub typ: String,
   pub val: String
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

            '(' =>  tokens.push( Token { typ: "parent".to_string(), val: "(".to_string() }),
            ')' =>  tokens.push( Token { typ: "parent".to_string(), val: ')'.to_string() }),
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
                 tokens.push( Token { typ: "number".to_string(), val: number });

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
                tokens.push( Token { typ: "string".to_string(), val: name });
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
                tokens.push( Token{ typ: "name".to_string(), val: name } );

            }

            c @ _ => {
                return Err(format!("Unexpected token: {}", c));
            }
            
        };

    }
    Ok(tokens)
}
