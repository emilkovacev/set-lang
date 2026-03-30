use std::io::{self, Read};

use regex::Regex;

/// A set of valid tokens within set-lang
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Token {
    Variable,
    Assignment,
    LBracket,
    RBracket,
    Newline,
    Print,
    Cardinality,
    ElementOf,
    Union,
    Intersection,
    Difference,
    CartesianProduct,
    Comment,
    Whitespace,
}

pub fn lexer<'a>(program: &'a String) -> Vec<(Token, &'a str)> {
    // The lexer table stores a list of regex matches against
    // input characters. Order of items in the lexer table matters,
    // the items at the top will be parsed before the items at the
    // bottom.
    let lexer_table = Vec::from([
        (r"^[{]$", Token::LBracket),
        (r"^[}]$", Token::RBracket),
        (r"^[\n]+$", Token::Newline),
        (r"^=$", Token::Assignment),
        (r"^PRINT$", Token::Print),
        (r"^CARDINALITY$", Token::Cardinality),
        (r"^ELEMENT\s+OF$", Token::ElementOf),
        (r"^UNION$", Token::Union),
        (r"^INTERSECTION$", Token::Intersection),
        (r"^DIFFERENCE$", Token::Difference),
        (r"^CARTESIAN\s+PRODUCT$", Token::CartesianProduct),
        (r"^[a-zA-Z0-9_]+$", Token::Variable),
        (r"^//.*$", Token::Comment),
        (r"^\s+$", Token::Whitespace),
    ]);
    let mut tokens = Vec::new();
    let mut prev_token: Option<(Token, &'a str)> = None;
    let mut starting = 0;
    let mut ending = 1;

    if program.len() == 0 {
        return tokens;
    }

    loop {
        let lexeme = &program[starting..ending];
        let token = check_token(lexeme, &lexer_table);
        match token {
            Some(t) => {
                prev_token = Some(t);
                ending += 1;
            }
            None => match prev_token {
                Some(t) => {
                    tokens.push(t);
                    starting = ending - 1
                }
                None => ending += 1,
            },
        }

        if ending > program.len() {
            match prev_token {
                Some(t) => tokens.push(t),
                None => {}
            }
            break;
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

#[cfg(test)]
mod test_lexer {
    use crate::{Token, lexer};

    fn cmp_tokens(expected: Vec<(Token, &str)>, actual: Vec<(Token, &str)>) {
        assert_eq!(
            expected.len(),
            actual.len(),
            "tokens have different numbers of elements {} != {}",
            expected.len(),
            actual.len()
        );
        for i in 0..expected.len() {
            let (expected_token, expected_str) = expected[i];
            let (actual_token, actual_str) = actual[i];
            assert_eq!(
                expected_token, actual_token,
                "Token {} does not match, {:?} != {:?}",
                i, expected_token, actual_token
            );
            assert_eq!(
                expected_str, actual_str,
                "String {} does not match, \"{}\" != \"{}\"",
                i, expected_str, actual_str
            )
        }
    }

    #[test]
    fn test_empty() {
        let program = String::from("");
        let expected = Vec::from([]);
        let actual = lexer(&program);
        cmp_tokens(expected, actual);
    }

    #[test]
    fn test_variable() {
        let program = String::from("VAR");
        let expected = Vec::from([(Token::Variable, "VAR")]);
        let actual = lexer(&program);
        cmp_tokens(expected, actual);
    }

    #[test]
    fn test_brackets() {
        let program = String::from("{}");
        let expected = Vec::from([(Token::LBracket, "{"), (Token::RBracket, "}")]);
        let actual = lexer(&program);
        cmp_tokens(expected, actual);
    }

    #[test]
    fn test_assignment() {
        let program = String::from("VAR = {}");
        let expected = Vec::from([
            (Token::Variable, "VAR"),
            (Token::Whitespace, " "),
            (Token::Assignment, "="),
            (Token::Whitespace, " "),
            (Token::LBracket, "{"),
            (Token::RBracket, "}"),
        ]);
        let actual = lexer(&program);
        cmp_tokens(expected, actual);
    }

    #[test]
    fn test_multiline() {
        let program = String::from("VAR = {}\nVAR_2 = {}");
        let expected = Vec::from([
            (Token::Variable, "VAR"),
            (Token::Whitespace, " "),
            (Token::Assignment, "="),
            (Token::Whitespace, " "),
            (Token::LBracket, "{"),
            (Token::RBracket, "}"),
            (Token::Newline, "\n"),
            (Token::Variable, "VAR_2"),
            (Token::Whitespace, " "),
            (Token::Assignment, "="),
            (Token::Whitespace, " "),
            (Token::LBracket, "{"),
            (Token::RBracket, "}"),
        ]);
        let actual = lexer(&program);
        cmp_tokens(expected, actual);
    }
}
