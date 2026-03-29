use std::io::{self, Read};

use regex::Regex;

#[derive(Debug, Copy, Clone)]
enum Token {
    Identifier,
    Literal,
    Separator,
    Operator,
    Comment,
    Whitespace,
}

fn lexer<'a>(program: &'a String) -> Vec<(Token, &'a str)> {
    // The lexer table stores a list of regex matches against
    // input characters. Order of items in the lexer table matters,
    // the items at the top will be parsed before the items at the
    // bottom.
    let lexer_table = Vec::from([
        (r"[{}]", Token::Literal),
        (r"[\n]+", Token::Separator),
        (r"=", Token::Operator),
        (r"PRINT", Token::Operator),
        (r"CARDINALITY", Token::Operator),
        (r"ELEMENT\s+OF", Token::Operator),
        (r"UNION", Token::Operator),
        (r"INTERSECTION", Token::Operator),
        (r"DIFFERENCE", Token::Operator),
        (r"CARTESIAN\s+PRODUCT", Token::Operator),
        (r"[a-zA-Z0-9_]+", Token::Identifier),
        (r"//.*$", Token::Comment),
        (r" ", Token::Whitespace),
    ]);
    let mut tokens = Vec::new();
    let mut starting = 0;
    let mut ending = 1;

    while ending <= program.len() {
        let lexeme = &program[starting..ending];
        let token = check_token(lexeme, &lexer_table);
        match token {
            Some(t) => {
                tokens.push(t);
                starting = ending;
                ending += 1;
            }
            None => ending += 1,
        }
    }
    return tokens;
}

fn check_token<'a>(lexeme: &'a str, lexer_table: &Vec<(&str, Token)>) -> Option<(Token, &'a str)> {
    for row in lexer_table.iter() {
        let re: &str = row.0;
        let re: Regex = Regex::new(re).unwrap();
        let token: Token = row.1;
        let lexeme_match = re.find(lexeme);
        match lexeme_match {
            Some(m) => {
                return Some((token, m.as_str()));
            }
            None => continue,
        }
    }
    return None;
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    println!("{:?}", lexer(&buffer));
}
