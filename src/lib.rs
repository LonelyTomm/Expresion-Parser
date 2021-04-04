use std::iter::Peekable;
use std::str::Chars;

enum LexNode {
    Num(char),
    Op(char),
    Paren(char)
}

struct ExprParser<'a> {
    chars: &'a str,
    lex_nodes: Vec<LexNode>
}

impl<'a> ExprParser<'a> {
    fn new(source_str: &'a str) -> ExprParser{
        ExprParser {
            chars: source_str,
            lex_nodes: vec![]
        }
    }

    fn parse(&mut self) -> Vec<LexNode> {
        let mut result = vec![];

        let mut peekable = self.chars.chars().peekable();
        while peekable.peek().is_some() {
            //skip_whitespaces(&mut peekable);
            let next_char = peekable.peek().unwrap();
            match next_char {
                '+' | '-' | '*' | '/' => {
                    result.push(LexNode::Op(*next_char));
                    peekable.next();
                },
                '0'..='9' => {
                    result.push(LexNode::Num(*next_char));
                    peekable.next();
                },
                '(' | ')' | '[' | ']' => {
                    result.push(LexNode::Paren(*next_char));
                    peekable.next();
                },
                ' ' => {
                    skip_whitespaces(&mut peekable)
                },
                _ => {
                    
                }
            }
        }

        result
    }
}

fn skip_whitespaces(source: &mut Peekable<Chars>) {
    while let Some(chr) = source.peek() {
        if *chr == ' ' {
            source.next();
        } else {
            break;
        }
    }
}

#[test]
fn test() {
    let mut testable = ExprParser::new("5 + 6 * 8 - (5 * 8)");
    let result = testable.parse();

    println!("Tested");
}

