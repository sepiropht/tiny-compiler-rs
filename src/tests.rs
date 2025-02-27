use crate::tokenizer::tokenizer;
use crate::parser::parser;
use crate::transformer::transformer;
use crate::code_generator::code_generator;

struct TestCase {
    input: &'static str,
    expected_output: &'static str,
}

#[test]
fn test_compiler_pipeline() {
    let test_cases = vec![
        TestCase {
            input: "(add 2 3)",
            expected_output: "add(2, 3)",
        },
        TestCase {
            input: "(subtract 5 2)",
            expected_output: "subtract(5, 2)",
        },
        TestCase {
            input: "(multiply 3 4)",
            expected_output: "multiply(3, 4)",
        },
        TestCase {
            input: "(divide 10 2)",
            expected_output: "divide(10, 2)",
        },
        TestCase {
            input: "(add 1 (subtract 3 2))",
            expected_output: "add(1, subtract(3, 2))",
        },
        TestCase {
            input: "(add (multiply 2 3) (divide 8 4))",
            expected_output: "add(multiply(2, 3), divide(8, 4))",
        },
        TestCase {
            input: "(log \"hello world\")",
            expected_output: "log(\"hello world\")",
        },
        TestCase {
            input: "(if (greater 3 2) (log \"yes\") (log \"no\"))",
            expected_output: "if(greater(3, 2), log(\"yes\"), log(\"no\"))",
        },
    ];

    for (i, test_case) in test_cases.iter().enumerate() {
        println!("Test case #{}: \"{}\"", i + 1, test_case.input);
        
        // Run the full pipeline
        let tokens = tokenizer(test_case.input).expect("Tokenization failed");
        println!("  Tokens: {:?}", tokens);
        
        let ast = parser(tokens).expect("Parsing failed");
        println!("  AST: {:?}", ast);
        
        let transformed_ast = transformer(ast).expect("Transformation failed");
        println!("  Transformed AST: {:?}", transformed_ast);
        
        let output = code_generator(&transformed_ast).expect("Code generation failed");
        println!("  Output: {}", output);
        
        assert_eq!(output, test_case.expected_output);
        println!("  ✓ Test passed\n");
    }
}

#[test]
fn test_error_handling() {
    // Test invalid syntax
    let invalid_input = "(add 2 3";
    let tokens_result = tokenizer(invalid_input);
    assert!(tokens_result.is_ok(), "Tokenizer should handle unclosed parentheses");
    
    // Test invalid token
    let invalid_token_input = "(add 2 @)";
    let tokens_result = tokenizer(invalid_token_input);
    assert!(tokens_result.is_err(), "Tokenizer should reject invalid tokens");
    
    // Test empty input
    let empty_input = "";
    let tokens_result = tokenizer(empty_input);
    assert!(tokens_result.is_ok(), "Tokenizer should handle empty input");
    let tokens = tokens_result.unwrap();
    assert!(tokens.is_empty(), "Empty input should produce empty tokens");
}

#[test]
fn test_complex_expressions() {
    let complex_input = "(add (multiply (subtract 10 5) 2) (divide 100 (add 5 5)))";
    let expected_output = "add(multiply(subtract(10, 5), 2), divide(100, add(5, 5)))";
    
    // Run the full pipeline
    let tokens = tokenizer(complex_input).expect("Tokenization failed");
    let ast = parser(tokens).expect("Parsing failed");
    let transformed_ast = transformer(ast).expect("Transformation failed");
    let output = code_generator(&transformed_ast).expect("Code generation failed");
    
    assert_eq!(output, expected_output);
}

#[test]
fn test_string_handling() {
    let input = "(concat \"hello\" \" \" \"world\")";
    let expected_output = "concat(\"hello\", \" \", \"world\")";
    
    // Run the full pipeline
    let tokens = tokenizer(input).expect("Tokenization failed");
    let ast = parser(tokens).expect("Parsing failed");
    let transformed_ast = transformer(ast).expect("Transformation failed");
    let output = code_generator(&transformed_ast).expect("Code generation failed");
    
    assert_eq!(output, expected_output);
} 