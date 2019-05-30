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

pub fn parser(tokens: Vec<Token>) -> Ast {
    unimplemented!()
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
            val: "2".to_string(),
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
                    val: "2".to_string(),
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
    assert!(parser(tokens) == ast);
}
