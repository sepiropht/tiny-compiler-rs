use crate::tokenizer::Token;

#[derive(PartialEq, Debug)]
pub struct Ast {
    typ: String,
    body: Vec<Node>,
}

#[derive(PartialEq, Debug)]
pub struct Node {
    typ: String,
    name: String,
    val: String,
    params: Vec<Node>,
}

pub fn parser(tokens: Vec<Token>) -> Result<Ast, String> {
    let mut iter = tokens.into_iter().peekable();
    let mut body = vec![];

    loop {
        let token = iter.next();

        if token.is_none() {
            break;
        }

        match walk(&mut iter, token.unwrap()) {
            Ok(node) => body.push(node),
            _ => break,
        };
    }

    let ast = Ast {
        typ: "Program".to_string(),
        body,
    };

    return Ok(ast);

    fn walk<I>(iter: &mut I, token: Token) -> Result<Node, String>
    where
        I: Iterator<Item = Token>,
    {
       //dbg!(&token);

        match &token.typ {
            typ if typ == "number" => {
                return Ok(Node {
                    typ: "NumberLiteral".to_string(),
                    name: "".to_string(),
                    val: token.val,
                    params: vec![],
                });
            }
            typ if typ == "string" => {
                return Ok(Node {
                    typ: "StringLiteral".to_string(),
                    name: "".to_string(),
                    val: token.val,
                    params: vec![],
                });
            }

            typ if typ == "paren" && token.val == "(" => {

                let token = iter.next().expect("2");
                //dbg!(&token);
                let mut params = vec![];
                let name = token.val.clone();
                let mut token = iter.next().expect("3");

                while (token.typ != "paren".to_string())
                    || (token.typ == "paren".to_string() && token.val != ")".to_string())
                {
                    params.push(walk(iter, token).expect("ici"));
                    token = match iter.next() {
                        Some(res) => res,
                        _ => Token {typ: "paren".to_string(), val: ")".to_string()}
                    }
                }
                // skip the next )
                //iter.next();

                return Ok(Node {
                    typ: "CallExpression".to_string(),
                    name,
                    val: "".to_string(),
                    params,
                });
            }
            _ => {
                return Err(format!(
                    "Syntax error: '''{}''' token is not valid",
                    token.typ
                ));
            }
        }
    };
}

#[test]
fn test_parser() {
    let tokens = vec![
        Token {
            typ: "paren".to_string(),
            val: "(".to_string(),
        },
        Token {
            typ: "name".to_string(),
            val: "add".to_string(),
        },
        Token {
            typ: "number".to_string(),
            val: "1".to_string(),
        },
        Token {
            typ: "paren".to_string(),
            val: "(".to_string(),
        },
        Token {
            typ: "name".to_string(),
            val: "subtract".to_string(),
        },
        Token {
            typ: "number".to_string(),
            val: "4".to_string(),
        },
        Token {
            typ: "number".to_string(),
            val: "2".to_string(),
        },
        Token {
            typ: "paren".to_string(),
            val: ")".to_string(),
        },
        Token {
            typ: "paren".to_string(),
            val: ')'.to_string(),
        },
    ];

    let ast = Ast {
        typ: "Program".to_string(),
        body: vec![Node {
            val: "".to_string(),
            typ: "CallExpression".to_string(),
            name: "add".to_string(),
            params: vec![
                Node {
                    name: "".to_string(),
                    typ: "NumberLiteral".to_string(),
                    val: "1".to_string(),
                    params: vec![],
                },
                Node {
                    val: "".to_string(),
                    typ: "CallExpression".to_string(),
                    name: "subtract".to_string(),
                    params: vec![
                        Node {
                            name: "".to_string(),
                            typ: "NumberLiteral".to_string(),
                            val: "4".to_string(),
                            params: vec![],
                        },
                        Node {
                            name: "".to_string(),
                            typ: "NumberLiteral".to_string(),
                            val: "2".to_string(),
                            params: vec![],
                        },
                    ],
                },
            ],
        }],
    };
    dbg!(parser(tokens.clone()).unwrap());
    assert!(parser(tokens).unwrap() == ast);
}
