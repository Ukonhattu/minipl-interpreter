use std::panic;

use crate::{language::lex::LexItem, data_structures::tree::ArenaTree};

pub struct SyntaxParser {
}

impl SyntaxParser {

    pub fn parse(&mut self, tokens: Vec<LexItem>) {

        let mut it = tokens.iter();
        let mut ast: ArenaTree<LexItem> = ArenaTree::default();

        while let Some(current) = it.next() {
            match current {
                LexItem::Var(t) => {
                    let var_token = ast.node(current.clone());
                    match  it.next() {
                        Some(x) => {
                            match x {
                                &LexItem::Identifier(_) => {
                                    let identifier = ast.node(x.clone());
                                    ast.arena[var_token].children.push(identifier);
                                }

                                _ => panic!("Unexpeccted token {token}, line {line}, column {column}, expected an identifier", token = t.text, line = t.line_number, column = t.column_number)
                            }
                        }

                        None => {
                            panic!("Unexpected EOF AFTER token {token}, line {line}, column {column}", token = t.text, line = t.line_number, column = t.column_number)
                        }
                    }
                }
                _ => {}
            }            
        }
    }


}