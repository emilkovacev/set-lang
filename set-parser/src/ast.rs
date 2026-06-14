use set_lexer::Token;

enum Node {
    Root,
    Assignment,
    SetInit,
}

struct AST {
    name: Node,
    parent: Option<Box<AST>>,
    left: Option<Box<AST>>,
    right: Option<Box<AST>>,
}

impl AST {
    fn new(
        name: Node,
        parent: Option<Box<AST>>,
        left: Option<Box<AST>>,
        right: Option<Box<AST>>,
    ) -> AST {
        AST {
            name: name,
            parent: parent,
            left: left,
            right: right,
        }
    }
}

fn parser(tokens: Vec<Token>) -> AST {
    let parser_rules = Vec::from([(Node::Assignment, Token::Variable, Token::)]);
    let mut root_ast = AST::new(Node::Root, None, None, None);
    for token in tokens {}
    root_ast
}
