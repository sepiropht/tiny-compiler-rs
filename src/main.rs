mod tokenizer;
mod parser;
mod transformer;
mod code_generator;
#[cfg(test)]
mod tests;

use crate::tokenizer::tokenizer;
use crate::parser::parser;
use crate::transformer::transformer;
use crate::code_generator::code_generator;

fn main() {
    // Example Lisp-like code to process
    let input = "(add 2 (subtract 4 2))";
    
    println!("Input: {}", input);
    
    // Tokenize the input
    match tokenizer(input) {
        Ok(tokens) => {
            println!("Tokens: {:?}", tokens);
            
            // Parse the tokens into an AST
            match parser(tokens) {
                Ok(ast) => {
                    println!("AST: {:?}", ast);
                    
                    // Transform the AST
                    match transformer(ast) {
                        Ok(transformed_ast) => {
                            println!("Transformed AST: {:?}", transformed_ast);
                            
                            // Generate code from the transformed AST
                            match code_generator(&transformed_ast) {
                                Ok(code) => {
                                    println!("Generated code: {}", code);
                                },
                                Err(err) => println!("Code generation error: {}", err),
                            }
                        },
                        Err(err) => println!("Transformation error: {}", err),
                    }
                },
                Err(err) => println!("Parsing error: {}", err),
            }
        },
        Err(err) => println!("Tokenization error: {}", err),
    }
}
