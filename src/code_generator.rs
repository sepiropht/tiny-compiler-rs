use crate::parser::Node;

pub fn code_generator(node: &Node) -> Result<String, String> {
    match node {
        // For ExpressionStatement, we'll join all the expressions with newlines
        Node::ExpressionStatement(body) => {
            let expressions: Result<Vec<String>, String> = body
                .iter()
                .map(|node| code_generator(node))
                .collect();
            
            Ok(expressions?.join("\n"))
        },
        
        // For Function calls, we'll generate the function name and arguments
        Node::Fun(name, args) => {
            // If this is a transformed node with Identifier and Arguments structure
            if args.len() == 2 && 
               matches!(&args[0], Node::Fun(n, _) if n == "Identifier") &&
               matches!(&args[1], Node::Fun(n, _) if n == "Arguments") {
                
                // Extract the actual function name from the Identifier node
                let function_name = match &args[0] {
                    Node::Fun(_, id_args) => {
                        if let Some(Node::String(name)) = id_args.get(0) {
                            name.clone()
                        } else {
                            return Err("Invalid Identifier node".to_string());
                        }
                    },
                    _ => return Err("Expected Identifier node".to_string()),
                };
                
                // Extract and generate code for the arguments
                let arguments = match &args[1] {
                    Node::Fun(_, arg_nodes) => {
                        let arg_strings: Result<Vec<String>, String> = arg_nodes
                            .iter()
                            .map(|node| code_generator(node))
                            .collect();
                        arg_strings?.join(", ")
                    },
                    _ => return Err("Expected Arguments node".to_string()),
                };
                
                Ok(format!("{}({})", function_name, arguments))
            } 
            // For non-transformed function nodes
            else {
                let arg_strings: Result<Vec<String>, String> = args
                    .iter()
                    .map(|node| code_generator(node))
                    .collect();
                
                Ok(format!("{}({})", name, arg_strings?.join(", ")))
            }
        },
        
        // For NumberLiteral nodes
        Node::Fun(name, args) if name == "NumberLiteral" => {
            if let Some(Node::Number(value)) = args.get(0) {
                Ok(value.to_string())
            } else {
                Err("Invalid NumberLiteral node".to_string())
            }
        },
        
        // For StringLiteral nodes
        Node::Fun(name, args) if name == "StringLiteral" => {
            if let Some(Node::String(value)) = args.get(0) {
                Ok(format!("\"{}\"", value))
            } else {
                Err("Invalid StringLiteral node".to_string())
            }
        },
        
        // For raw Number nodes (untransformed)
        Node::Number(value) => Ok(value.to_string()),
        
        // For raw String nodes (untransformed)
        Node::String(value) => Ok(format!("\"{}\"", value)),
        
        // If we haven't recognized the node type
        _ => Err(format!("Unrecognized node type in code generator")),
    }
}

#[test]
fn test_code_generator() {
    use crate::parser::Node;
    
    // Create a simple transformed AST
    let transformed_ast = Node::ExpressionStatement(vec![Node::Fun(
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
    
    // Expected output
    let expected = "add(2, subtract(4, 2))";
    
    assert_eq!(code_generator(&transformed_ast).unwrap(), expected);
} 