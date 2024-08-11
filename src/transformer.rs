use crate::parser::Node;

pub fn transformer(ast: Node) -> Result<Node, String> {
    match ast {
        Node::ExpressionStatement(body) => {
            let transformed_body = transform_nodes(body)?;
            Ok(Node::ExpressionStatement(transformed_body))
        }
        _ => Err("Expected ExpressionStatement as root node".to_string()),
    }
}

fn transform_nodes(nodes: Vec<Node>) -> Result<Vec<Node>, String> {
    nodes.into_iter().map(transform_node).collect()
}

fn transform_node(node: Node) -> Result<Node, String> {
    match node {
        Node::Fun(name, params) => {
            let transformed_params = transform_nodes(params)?;
            Ok(Node::Fun(
                name.clone(),
                vec![
                    Node::Fun("Identifier".to_string(), vec![Node::String(name.clone())]),
                    Node::Fun("Arguments".to_string(), transformed_params),
                ],
            ))
        }
        Node::Number(value) => Ok(Node::Fun(
            "NumberLiteral".to_string(),
            vec![Node::Number(value)],
        )),
        Node::String(value) => Ok(Node::Fun(
            "StringLiteral".to_string(),
            vec![Node::String(value)],
        )),
        Node::ExpressionStatement(_) => {
            Err("Unexpected ExpressionStatement in transform_node".to_string())
        }
    }
}

#[test]
fn test_transformer() {
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

    let expected_ast = Node::ExpressionStatement(vec![Node::Fun(
        "add".to_string(),
        vec![
            Node::Fun(
                "Identifier".to_string(),
                vec![Node::String("add".to_string())],
            ),
            Node::Fun(
                "Arguments".to_string(),
                vec![
                    Node::Fun("NumberLiteral".to_string(), vec![Node::Number(2)]),
                    Node::Fun(
                        "subtract".to_string(),
                        vec![
                            Node::Fun(
                                "Identifier".to_string(),
                                vec![Node::String("subtract".to_string())],
                            ),
                            Node::Fun(
                                "Arguments".to_string(),
                                vec![
                                    Node::Fun("NumberLiteral".to_string(), vec![Node::Number(4)]),
                                    Node::Fun("NumberLiteral".to_string(), vec![Node::Number(2)]),
                                ],
                            ),
                        ],
                    ),
                ],
            ),
        ],
    )]);

    assert_eq!(transformer(ast), Ok(expected_ast));
}
