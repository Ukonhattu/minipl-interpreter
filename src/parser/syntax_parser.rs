
use std::mem::{self, Discriminant};
use std::{panic, any::Any};
use std::collections::HashMap;
use crate::language::lex::LexItemInfo;
use crate::{language::{lex::{LexItem}, ast::{VariableInfo, VariableType, ConstantInfo, BinOpType}}, data_structures::tree::ArenaTree};
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
            tokens,
            variables: HashMap::new(),
            ast: ArenaTree::default()
        }
    }

    pub fn parse(&mut self) -> ArenaTree<AstItem> {
        self.parse_stmts()
    }

    fn parse_stmts(&mut self) -> ArenaTree<AstItem>{
        let mut index = 0;
        let root = self.ast.node(AstItem::Root);
        while index < self.tokens.len() {
            let statement = self.parse_stmt(index);
            if statement.0 != None {
                self.ast.arena[root].children.push(statement.0.unwrap());
                self.ast.arena[statement.0.unwrap()].parent = Some(root);
                index = statement.1 + 1;
                continue;
            }

            panic!("Error in parsing statement that starts at {:#?} ", &self.tokens[index]);
        }
        self.ast.clone()
    }

    fn parse_block(&mut self, index: usize, end: Discriminant<LexItem>) -> (Option<usize>, usize) {
        let mut i = index;
        let block_node = self.ast.node(AstItem::Block);
        while i < self.tokens.len() {
            if let LexItem::End(_) = &self.tokens[i] {
                i += 1;       
                if mem::discriminant(&self.tokens[i]) == end {
                    i += 1;
                    if let LexItem::StatementEnd(_) = &self.tokens[i] {
                        return (Some(block_node), i);
                    } else {
                         panic!("Expected semicolon, found {:#?}", self.tokens[i]);
                    }
                }    
            }

            let statement = self.parse_stmt(i);
            if statement.0 != None {
                self.ast.arena[block_node].children.push(statement.0.unwrap());
                self.ast.arena[statement.0.unwrap()].parent = Some(block_node);
                i = statement.1 + 1;
                continue;
            }
            panic!("Error in parsing statement that starts at {:#?} ", &self.tokens[i]);
        }
        panic!("ERROR end for {:#?} not found, EOF", end);
    }

    fn parse_stmt(&mut self, index: usize) -> (Option<usize>, usize) {
        let assigment = self.parse_assigment(index);
        if assigment.0 != None {
            return assigment;
        }
        let read = self.parse_read(index);
        if read.0 != None {
            return read;
        }
        let print = self.parse_print(index);
        if print.0 != None {
            return print;
        }
        let assert = self.parse_assert(index);
        if assert.0 != None {
            return assert;
        }
        let parse_for = self.parse_for(index);
        if parse_for.0 != None {
            return parse_for;
        }
        (None, index)
    }

    fn parse_expr(&mut self, index: usize) -> (Option<usize>, usize) { // <opnd> <op> <opnd> | [<Not>] <opnd>
        let mut i = index;
        let mut has_unary_opnd = false;
        if let LexItem::Not(_) = &self.tokens[i] {
            has_unary_opnd = true;
            i += 1;
        }
        let opnd = self.parse_opnd(i);
        if opnd.0 == None {
            return (None, index)
        } else {
            i = opnd.1;
        }
        if has_unary_opnd {
            let not_item = AstItem::Not;
            let not_node = self.ast.node(not_item);
            self.ast.arena[not_node].children.push(opnd.0.unwrap());
            return (Some(not_node), i)
        }
        i += 1;
        if i >= self.tokens.len() {
            return (opnd.0, i-1)
        }
        // Does it have <op>
        let op_type;
        match &self.tokens[i] {
            LexItem::Plus(_) => {
                op_type = BinOpType::Plus;
            }
            LexItem::Minus(_) => {
                op_type = BinOpType::Minus;
            }
            LexItem::Star(_) => {
                op_type = BinOpType::Multiply;
            }
            LexItem::Slash(_) => {
                op_type = BinOpType::Divide;
            }
            LexItem::LessThan(_) => {
                op_type = BinOpType::LessThan;
            }
            LexItem::Equal(_) => {
                op_type = BinOpType::Equal;
            }
            LexItem::And(_) => {
                op_type = BinOpType::And;
            }
            _ => {
                return (opnd.0, i-1)
            }
        }
        i += 1;
        let second_opnd = self.parse_opnd(i); // Return op_node -> (opnd, second_opnd)
        if second_opnd.0 != None {
            let op_item = AstItem::BinOp(op_type);
            let op_node = self.ast.node(op_item);
            self.ast.arena[op_node].children.push(opnd.0.unwrap());
            self.ast.arena[op_node].children.push(second_opnd.0.unwrap());
            self.ast.arena[opnd.0.unwrap()].parent = Some(op_node);
            self.ast.arena[second_opnd.0.unwrap()].parent = Some(op_node);
            return (Some(op_node), second_opnd.1)
        }

        (None, index)
    }

    fn parse_opnd(&mut self, index: usize) -> (Option<usize>, usize) { // <int_literal> | <string_literal> | <var_identifier> | "(" <expr> ")"
        let mut i = index;

        if let LexItem::IntegerLiteral(t) = &self.tokens[i] {
            let constant_item = AstItem::Constant(ConstantInfo {value: t.text.clone(), const_type: VariableType::Int});
            let constant_node = self.ast.node(constant_item);
            return (Some(constant_node), i)
        } else if let LexItem::StringLiteral(t) = &self.tokens[i] {
            let constant_item = AstItem::Constant(ConstantInfo {value: t.text.clone(), const_type: VariableType::String});
            let constant_node = self.ast.node(constant_item);
            return (Some(constant_node), i)
        }else if let LexItem::BoolTrue(t) = &self.tokens[i] {
            let constant_item = AstItem::Constant(ConstantInfo {value: t.text.clone(), const_type: VariableType::Bool});
            let constant_node = self.ast.node(constant_item);
            return (Some(constant_node), i)
        }else if let LexItem::BoolFalse(t) = &self.tokens[i] {
            let constant_item = AstItem::Constant(ConstantInfo {value: t.text.clone(), const_type: VariableType::Bool});
            let constant_node = self.ast.node(constant_item);
            return (Some(constant_node), i)
        } else if let LexItem::Identifier(t) = &self.tokens[i] {
            let variable_info;
            if self.variables.contains_key(&t.text.clone()) {
                variable_info = self.variables.get(&t.text.clone())
            } else {
                panic!("ERROR use of uninitialized variable, line {line}, column {column}", line = t.line_number, column = t.column_number);
            }
            let variable_item = AstItem::Variable(variable_info.unwrap().clone());
            let variable_node = self.ast.node(variable_item);
            return (Some(variable_node), i)
        }

        if let LexItem::LeftParen(_) = &self.tokens[i] {
            i += 1;
            let expr = self.parse_expr(i);
            if expr.0 == None {
                panic!("Expected expression after {:#?}", &self.tokens[i]);
            }
            i = expr.1 + 1;
            if let LexItem::RightParen(_) = &self.tokens[i] {
                return (expr.0, i)
            } else {
                panic!("Expected ')', found {:#?}", &self.tokens[i]);
            }
        }

        (None, index)
    }

    pub fn parse_for(&mut self, index: usize) -> (Option<usize>, usize) {
        let mut i = index;
        let variable_info;

        if let LexItem::For(_) = &self.tokens[i] {
             i += 1;
        } else {
            return (None, index)
        }
        if let LexItem::Identifier(t) = &self.tokens[i] {
            i += 1;
            if self.variables.contains_key(&t.text) {
                variable_info = self.variables.get(&t.text).unwrap().clone();
            } else {
                panic!("ERROR undefined variable {} at line {}, column {}", t.text, t.line_number, t.column_number);
            }
        } else {
            panic!("ERROR Expected identifier, found {:#?}", self.tokens[i]);
        }
        if let LexItem::In(_) = &self.tokens[i] {
            i += 1;
        } else {
            panic!("ERROR Expected keyword in, found {:#?}", self.tokens[i]);
        }
        let expr_left = self.parse_expr(i);
        if expr_left.0 != None {
            i = expr_left.1 + 1;
        } else {
            panic!("Expected expression after {:#?}", self.tokens[i]);
        }
        if let LexItem::Range(_) = &self.tokens[i] {
            i += 1;
        } else {
            panic!("ERROR expected range operator \"..\", found {:#?}", self.tokens[i]);
        }
        let expr_right = self.parse_expr(i);
        if expr_right.0 != None {
            i = expr_right.1 + 1;
        } else {
            panic!("Expected expression after {:#?}", self.tokens[i]);
        }
        if let LexItem::Do(_) = &self.tokens[i] {
            i += 1;
        } else {
            panic!("ERROR Expected keyword do, found {:#?}", self.tokens[i]);
        }
        let block_node = self.parse_block(i, mem::discriminant(&LexItem::For(LexItemInfo{text: "a".into(), line_number: -1, column_number: -1}))); // TODO to this smarter lmao (I want the type of LexItem::For to give as a parameter)
        i = block_node.1;
        let for_node = self.ast.node(AstItem::For);
        let variable_node = self.ast.node(AstItem::Variable(variable_info));
        let range_node = self.ast.node(AstItem::Range);

        self.ast.arena[range_node].children.push(expr_left.0.unwrap());
        self.ast.arena[range_node].children.push(expr_right.0.unwrap());
        self.ast.arena[expr_left.0.unwrap()].parent = Some(range_node);
        self.ast.arena[expr_right.0.unwrap()].parent = Some(range_node);

        self.ast.arena[for_node].children.push(variable_node);
        self.ast.arena[for_node].children.push(range_node);
        self.ast.arena[for_node].children.push(block_node.0.unwrap());
        self.ast.arena[variable_node].parent = Some(for_node);
        self.ast.arena[range_node].parent = Some(for_node);
        self.ast.arena[block_node.0.unwrap()].parent = Some(for_node);
        (Some(for_node), i)
    }

    //-------------------------------------------------------------------------------
    
    pub fn parse_read(&mut self, index: usize) -> (Option<usize>, usize) {
        let mut i = index;

        if let LexItem::Read(_) = &self.tokens[i] {
            i += 1;
        } else {
            return (None, index)
        }
        let variable_info;
        if let LexItem::Identifier(t) = &self.tokens[i] {
            if self.variables.contains_key(&t.text) {
                variable_info = self.variables.get(&t.text).unwrap().clone();
            } else {
                panic!("Error! undefined variable {:#?}", self.tokens[i]);
            }
        } else {
            panic!("ERROR expexted identifier after {:#?}", self.tokens[i])
        }
        i += 1;
        if let LexItem::StatementEnd(_) = &self.tokens[i] {
            let read_item = AstItem::Read;
            let read_note = self.ast.node(read_item);
            let variable_item = AstItem::Variable(variable_info);
            let variable_node = self.ast.node(variable_item);
            self.ast.arena[read_note].children.push(variable_node);
            self.ast.arena[variable_node].parent = Some(read_note);
            (Some(read_note), i)
        } else {
            panic!("Expected semicolon, found {:#?}", self.tokens[i]);
        }
    }

    fn parse_assert(&mut self, index: usize) -> (Option<usize>, usize) {
        let mut i = index;

        if let LexItem::Assert(_) = &self.tokens[i] {
            i += 1;
        } else {
            return (None, index)
        }

        if let LexItem::LeftParen(_) = &self.tokens[i] {
            i += 1;
            let expr = self.parse_expr(i);
            if expr.0 == None {
                panic!("Expected expression after {:#?}", &self.tokens[i]);
            }
            i = expr.1 + 1;
            if let LexItem::RightParen(_) = &self.tokens[i] {
                let assert_item = AstItem::Assert;
                let assert_node = self.ast.node(assert_item);
                self.ast.arena[assert_node].children.push(expr.0.unwrap());
                self.ast.arena[expr.0.unwrap()].parent = Some(assert_node);
                i += 1;
                if let LexItem::StatementEnd(_) = &self.tokens[i] {
                    (Some(assert_node), i)
                } else {
                    panic!("ERROR expected ';', found {:#?}", self.tokens[i]);
                }
                
            } else {
                panic!("Expected ')', found {:#?}", &self.tokens[i]);
            }
        } else {
            panic!("ERROR expected '(', found {:#?}", self.tokens[i]);
        }
    }

    fn parse_print(&mut self, index: usize) -> (Option<usize>, usize) {
        let mut i = index;
        
        if let LexItem::Print(_) = &self.tokens[i] {
            i += 1;
        } else {
            return (None, index)
        }

        let expr = self.parse_expr(i);
        if expr.0 == None {
            panic!("Expected an expression after print at {:#?}", self.tokens[i-1]);
        } else {
            i = expr.1;
        }
        i += 1;
        
        if let LexItem::StatementEnd(_) = &self.tokens[i]{
            let print_item = AstItem::Print;
            let print_node = self.ast.node(print_item);
            self.ast.arena[print_node].children.push(expr.0.unwrap());
            (Some(print_node), i)
        } else {
            panic!("Expected semicolon, found {:#?}", self.tokens[i]);
        }
    }


    fn parse_assigment(&mut self, index: usize) -> (Option<usize>, usize) { // "var" <var_ident> ":" <type> [":=" <expr>] | <var_ident> ":=" <expr>
        let mut first_assign = false;
        let mut i = index;
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
            } else if !self.variables.contains_key(&t.text) {
                panic!("ERROR undefined variable {var_name},  line {line}, column {column}", var_name = &t.text ,line = t.line_number, column = t.column_number)
            }
            var_name = t.text.clone();
            i += 1;
        } else {
            return (None, index)
        }

        if first_assign {
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
            }
        } else {
            match self.variables.get(&var_name) {
                Some(t) => {
                    var_type = t.var_type.clone();
                }
                None => panic!("Unexpected ERROR variable {var_name} is defined but not found!") // This shouldn't ever happen 
            }
        }


        if let LexItem::Assign(_) = &self.tokens[i] {
            i += 1;
        } else {
            return (None, index)
        }

        let expr = self.parse_expr(i);
        if expr.0 != None {
            i = expr.1 + 1;
            if let LexItem::StatementEnd(_) = &self.tokens[i] {
                let expr_index = expr.0.unwrap();
                let assign = self.make_assigment_node_constant(var_name, None, var_type);
                self.ast.arena[assign].children.push(expr_index);
                self.ast.arena[expr_index].parent = Some(assign);
                return (Some(assign), i)
    
            } else {
                panic!("Expected semicolon after {:#?}", self.tokens[i])
            }
        }       

        (None, index)
    }

    fn make_assigment_node_constant(&mut self, name: String, value: Option<String>, var_type: VariableType) -> usize {
        let assign_item: AstItem = AstItem::Assign;
        let assign = self.ast.node(assign_item);
        let variable_item = AstItem::Variable(VariableInfo{name, var_type: var_type.clone()});
        let variable = self.ast.node(variable_item);

        self.ast.arena[assign].children.push(variable);
        self.ast.arena[variable].parent = Some(assign);
        if value != None {
            let value_item = AstItem::Constant(ConstantInfo {value: value.unwrap_or_default(), const_type: var_type});
            let value_node = self.ast.node(value_item);
            self.ast.arena[assign].children.push(value_node);
            self.ast.arena[value_node].parent = Some(assign);
        }
        assign
    }



}
