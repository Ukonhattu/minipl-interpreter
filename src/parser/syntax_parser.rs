use std::{panic, env::var};
use std::collections::HashMap;
use crate::{language::{lex::{LexItem}, ast::{VariableInfo, VariableType, SourceInfo, ConstantInfo}}, data_structures::tree::ArenaTree};
use crate::language::ast::AstItem;

pub struct SyntaxParser {
    tokens: Vec<LexItem>,
    variables: HashMap<String, VariableInfo>,
    ast: ArenaTree<AstItem>
}

impl SyntaxParser {

    pub fn new(tokens: Vec<LexItem>) -> Self{
        if tokens.len() <= 0 {
            panic!("ERROR Token stream empty!");
        }
        Self {
            tokens: tokens,
            variables: HashMap::new(),
            ast: ArenaTree::default()
        }
    }

    pub fn parse(&mut self) -> ArenaTree<AstItem>{
        let mut index = 0;
        let root = self.ast.node(AstItem::Root);
        while index < self.tokens.len() {
            let statement = self.parse_stmt(index);
            if statement.0 != None {
                self.ast.arena[root].children.push(statement.0.unwrap());
                self.ast.arena[statement.0.unwrap()].parent = Some(root);
                index = statement.1;
                continue;
            }
            index += 1;
        }
        self.ast.clone()
    }

    fn parse_stmt(&mut self, index: usize) -> (Option<usize>, usize) {
        let assigment = self.parse_assigment(index);
        if assigment.0 != None {
            return assigment;
        } 
        (None, index)
    }

    fn parse_expr(&self, index: usize) -> (Option<usize>, usize) {
        (None, index)
    }

    fn parse_opnd(&self, index: usize) -> Option<usize> {
        None
    }

    fn parse_type(&self, index: usize) -> Option<usize> {
        None
    }

    //-------------------------------------------------------------------------------

    fn parse_assigment(&mut self, index: usize) -> (Option<usize>, usize) {
        let mut first_assign = false;
        let mut i = index.clone();
        let var_type: VariableType;
        let var_name: String;
        if let LexItem::Var(_) = &self.tokens[i] {
            first_assign = true;
            i += 1;
        }
        if let LexItem::Identifier(t) = &self.tokens[i] {
            if first_assign {
                if self.variables.contains_key(&t.text) {
                    panic!("ERROR variable name already defined, line {line}, column {column}", line = t.line_number, column = t.column_number)
                }  
            }
            var_name = t.text.clone();
            i += 1;
        } else {
            return (None, index)
        }

        if let LexItem::Separator(_) = &self.tokens[i] {
            i += 1;
        } else {
            return (None, index)
        }

        if let LexItem::String(_) = &self.tokens[i] {
            var_type = VariableType::String;
            i += 1;
        } else if let LexItem::Bool(_) = &self.tokens[i] {
            var_type = VariableType::Bool;
            i += 1;
        } else if let LexItem::Int(_) = &self.tokens[i] {
            var_type = VariableType::Int;
            i += 1;
        } else {
            return (None, index)
        }
        self.variables.insert(var_name.clone(), VariableInfo {name: var_name.clone(), var_type: var_type.clone()});
        if let LexItem::StatementEnd(_) = &self.tokens[i] {
            return (Some(self.make_assigment_node_constant(var_name,None, var_type)), i);
        } else if let LexItem::Assign(_) = &self.tokens[i] {
            i += 1;
        }


        (None, index)
    }

    fn make_assigment_node_constant(&mut self, name: String, value: Option<String>, var_type: VariableType) -> usize {
        let assign_item: AstItem = AstItem::Assign;
        let assign = self.ast.node(assign_item);
        let variable_item = AstItem::Variable(VariableInfo{name, var_type});
        let variable = self.ast.node(variable_item);

        self.ast.arena[assign].children.push(variable);
        self.ast.arena[variable].parent = Some(assign);
        if value != None {
            let value_item = AstItem::Constant(ConstantInfo {value: value.unwrap_or_default()});
            let value_node = self.ast.node(value_item);
            self.ast.arena[assign].children.push(value_node);
            self.ast.arena[value_node].parent = Some(assign);
        }
        assign
    }



}