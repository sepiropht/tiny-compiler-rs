use crate::tokenizer::Token;

#[derive(PartialEq, Debug)]
pub enum Node {
    Fun(String, Vec<Node>),
    Number(i32),
    String(String),
    ExpressionStatement(Vec<Node>),
}

pub fn parser(tokens: Vec<Token>) -> Result<Node, String> {
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

    let ast = Node::ExpressionStatement(body);
    dbg!(&ast);
    return Ok(ast);
}

fn walk<I>(iter: &mut I, token: Token) -> Result<Node, String>
where
    I: Iterator<Item = Token>,
{
    //dbg!(&token);

    match &token {
        Token::Number(num) => Ok(Node::Number(*num)),
        Token::String(ch) => Ok(Node::String(ch.to_string())),

        Token::OpenParen => {
            let token = iter.next().expect("2");
            //dbg!(&token);
            let mut params = vec![];

            let name = match token {
                Token::Identifier(value) => value,
                _ => "".to_string(),
            };

            let mut token = iter.next().expect("3");

            while token != Token::CloseParen {
                params.push(walk(iter, token).expect("ici"));
                token = match iter.next() {
                    Some(res) => res,
                    _ => Token::CloseParen,
                }
            }
            // skip the next )
            //iter.next();

            return Ok(Node::Fun(name, params));
        }
        _ => {
            return Err("Syntax error: '''{}''' token is not valid".to_string());
        }
    }
}

#[test]
fn test_parser() {
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
    let ast = Node::ExpressionStatement(vec![Node::Fun(
        "add".to_string(),
        vec![
            Node::Number(2),
            Node::Fun(
                "subtract".to_string(),
                vec![Node::Number(4), Node::Number(2)],
            ),
        ],
    )]);

    dbg!(parser(tokens.clone()).unwrap());
    assert!(parser(tokens) == Ok(ast));
}
