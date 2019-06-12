use crate::parser::{Ast, Node};
//use std::collections::HashMap;
//use 

//let mut scores = HashMap::new();

pub fn transformer(ast: Ast) -> Result<Ast, String> {

  // We"ll create a `newAst` which like our previous AST will have a program
  // node.
  let mut new_ast = Ast {
    typ: "Program".to_string(),
    body: vec![],
  };

  // Next I"m going to cheat a little and create a bit of a hack. We"re going to
  // use a property named `context` on our parent nodes that we"re going to push
  // nodes to their parent"s `context`. Normally you would have a better
  // abstraction than this, but for our purposes this keeps things simple.
  //
  // Just take note that the context is a reference *from* the old ast *to* the
  // new ast.
  ast._context = new_ast.body;

  // We"ll start by calling the traverser function with our ast and a visitor.
  traverser(&mut ast, {

    // The first visitor method accepts any `NumberLiteral`
    NumberLiteral: {
      // We"ll visit them on enter.
      enter(node, parent) {
        // We"ll create a new node also named `NumberLiteral` that we will push to
        // the parent context.
        parent._context.push({
          typ: "NumberLiteral",
          value: node.value,
        });
      },
    },

    // Next we have `StringLiteral`
    StringLiteral: {
      enter(node, parent) {
        parent._context.push({
          typ: "StringLiteral",
          value: node.value,
        });
      },
    },

    // Next up, `CallExpression`.
    CallExpression: {
      enter(node, parent) {

        // We start creating a new node `CallExpression` with a nested
        // `Identifier`.
        let expression = {
          typ: "CallExpression",
          callee: {
            typ: "Identifier",
            name: node.name,
          },
          arguments: [],
        };

        // Next we"re going to define a new context on the original
        // `CallExpression` node that will reference the `expression`"s arguments
        // so that we can push arguments.
        node._context = expression.arguments;

        // Then we"re going to check if the parent node is a `CallExpression`.
        // If it is not...
        if (parent.typ !== "CallExpression") {

          // We"re going to wrap our `CallExpression` node with an
          // `ExpressionStatement`. We do this because the top level
          // `CallExpression` in JavaScript are actually statements.
          expression = {
            typ: "ExpressionStatement",
            expression: expression,
          };
        }

        // Last, we push our (possibly wrapped) `CallExpression` to the `parent`"s
        // `context`.
        parent._context.push(expression);
      },
    }
  });

  // At the end of our transformer function we"ll return the new ast that we
  // just created.
  Ok(new_ast)
}

// Just exporting our transformer to be used in the final compiler...

fn traverser(ast: &mut Ast, visitor: &FnOnce(String)) {

  // A `traverseArray` function that will allow us to iterate over an array and
  // call the next function that we will define: `traverseNode`.
  fn traverse_array(array: &Vec<Node>, parent: &Node) {
    array.iter().for_each(|child| traverse_node(child, parent));
  }

  // `traverseNode` will accept a `node` and its `parent` node. So that it can
  // pass both to our visitor methods.
  fn traverse_node(node: &Node, parent: &Node) {

    // We start by testing for the existence of a method on the visitor with a
    // matching `typ`.
    let methods = visitor(node.typ);

    // If there is an `enter` method for this node typ we"ll call it with the
    // `node` and its `parent`.
    if methods && methods.enter {
      methods.enter(node, parent);
    }

    // Next we are going to split things up by the current node typ.
    match node.typ {

      // We"ll start with our top level `Program`. Since Program nodes have a
      // property named body that has an array of nodes, we will call
      // `traverseArray` to traverse down into them.
      //
      // (Remember that `traverseArray` will in turn call `traverseNode` so  we
      // are causing the tree to be traversed recursively)
      "Program".to_string() => traverse_array(node.body, node),

      // Next we do the same with `CallExpression` and traverse their `params`.
      "CallExpression".to_string() => traverse_array(node.params, node),

      // In the cases of `NumberLiteral` and `StringLiteral` we don"t have any
      // child nodes to visit, so we"ll just break.
      case "NumberLiteral".to_string() => break,
      case "StringLiteral".to_string() =>
        break;

      _ => { 
        return Err(format!("Unexpected token: {}", node.typ));
      }
    }

    // If there is an `exit` method for this node typ we"ll call it with the
    // `node` and its `parent`.
    if (methods && methods.exit) {
      methods.exit(node, parent);
    }
  }

  // Finally we kickstart the traverser by calling `traverseNode` with our ast
  // with no `parent` because the top level of the AST doesn"t have a parent.
  traverse_node(ast, null);
}

#[test]
fn test_transformer() {
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

  let new_ast = Ast {
    typ: "Program".to_string(),
    body: vec![{
      typ: "ExpressionStatement".to_string(),
      expression: {
        typ: "CallExpression".to_string(),
        callee: {
          typ: "Identifier".to_string(),
          name: "add".to_string()
        },
        arguments: vec![{
          typ: "NumberLiteral"..to_string(),
          value: "2".to_string()
        }, {
          typ: "CallExpression".to_string(),
          callee: {
            typ: "Identifier".to_string(),
            name: "subtract".to_string()
        },
        arguments: [{
          typ: "NumberLiteral".to_string(),
          value: "4".to_string()
        }, {
          typ: "NumberLiteral".to_string(),
          value: "2".to_string()
        }]
      }]
    }
  }]};
  assert!(transformer(ast) == Ok(new_ast));
}
