use core::fmt;
use std::{collections::HashMap};

use crate::{data_structures::tree::{ArenaTree, Node}, language::ast::{AstItem, VariableType, BinOpType}};

pub struct Interpreter {
    ast: ArenaTree<AstItem>,
    variables: HashMap<String, RunTimeVariable>
}

impl Interpreter {

    pub fn new(ast: ArenaTree<AstItem>) -> Self {
        Self {
            ast: ast,
            variables: HashMap::new()
        }
    }
    pub fn interpret(&mut self) {
        //let root = self.ast.arena[0].clone();
        let children = self.ast.arena[0].children.clone();
        for child in children {
            let node = self.ast.arena[child].clone();
            self.parse_node(node);
        }
    }

    fn parse_node(&mut self, node: Node<AstItem>) {
        match node.val {
            AstItem::Print => {
                let child = self.ast.arena[node.children[0]].clone();
                let value = self.expect_expr(child);
                print!("{}", value);
            }
            AstItem::Assign => {
                self.handle_assign(node);
            }
            AstItem::Read => {
                self.handle_read(node);
            }
            AstItem::Assert => {
                self.handle_assert(node);
            }
            AstItem::For => {
                self.handle_for(node);
            }
            _ => panic!("Unexpected node {:#?}", node)
        }
    }

    fn handle_for(&mut self, node: Node<AstItem>) {
        panic!("NOT IMPLEMENTED")
    }

    fn handle_assert(&mut self, node: Node<AstItem>) {
        panic!("NOT IMPLEMENTED")
    }

    fn handle_read(&mut self, node: Node<AstItem>) {
        let child = self.ast.arena[node.children[0]].clone();
        let var_name;
        let var_type;
        let var;
        match child.val {
            AstItem::Variable(t) => {
                var_name = t.name;
                var_type = t.var_type;
            }
            _ => panic!("ERROR trying to assign value to a non variable")
        }
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        match var_type {
            VariableType::Int => {
                var = RunTimeVariable{name: var_name.clone(), var_type: var_type, value: Value::Int(line.trim().parse().unwrap())}
            }
            VariableType::String => {
                trim_newline(&mut line);
                var = RunTimeVariable{name: var_name.clone(), var_type: var_type, value: Value::String(line)}
            }
            VariableType::Bool => panic!("Cannot read a boolean value")
        }
        if self.variables.contains_key(&var_name) {
            self.variables.insert(var_name, var);
        } else {
            panic!("Trying to read to an undefined variable")
        }
    }

    fn handle_assign(&mut self, node: Node<AstItem>) {
        let left_child = self.ast.arena[node.children[0]].clone();  
        let var_name;
        let var_type;
        let var_value;
        let var;
        match left_child.val {
            AstItem::Variable(t) => {
                var_name = t.name;
                var_type = t.var_type;
            }
            _ => panic!("ERROR trying to assign value to a non variable")
        }
        if node.children.len() > 1 {
            let right_child = self.ast.arena[node.children[1]].clone();
            var_value = self.expect_expr(right_child);
            var = RunTimeVariable {name: var_name.clone(), var_type: var_type, value: var_value};    
        } else {
            var = RunTimeVariable {name: var_name.clone(), var_type: var_type, value: Value::NULL};
        }
        self.variables.insert(var_name, var);
    }

    fn expect_expr(&mut self, node: Node<AstItem>) -> Value {
        if node.children.len() == 0 {
            //let child = self.ast.arena[node.children[0]].clone();
            return self.expect_opnd(node)
        }
        match node.val.clone() {
            AstItem::BinOp(t) => {
                match t {
                    BinOpType::And => return self.handle_and(node),
                    BinOpType::Divide => return self.handle_divide(node),
                    BinOpType::Equal => return self.handle_equal(node),
                    BinOpType::LessThan => return self.handle_less_than(node),
                    BinOpType::Minus => return self.handle_minus(node),
                    BinOpType::Multiply => return self.handle_multiply(node),  
                    BinOpType::Plus => return self.handle_plus(node),           
                }
            }
            _ => panic!("Error, unexpected node {:#?}", node)
        }
    }
    
    fn handle_plus(&mut self, node: Node<AstItem>) -> Value {
        let left_child = self.ast.arena[node.children[0]].clone();
        let right_child = self.ast.arena[node.children[1]].clone();
        let left_side = self.expect_expr(left_child);
        let right_side = self.expect_expr(right_child);
        
        match (left_side, right_side) {
            (Value::Int(t), Value::Int(v)) => {
                let sum = t + v;
                Value::Int(sum)
            }
            (Value::String(t), Value::String(v)) => {
                let mut concat = String::new();
                concat.push_str(&t);
                concat.push_str(&v);
                Value::String(concat)
            }
            _ => panic!("Incomtabile add of two values")
        }
    }
    fn handle_minus(&mut self, node: Node<AstItem>) -> Value {
        let left_child = self.ast.arena[node.children[0]].clone();
        let right_child = self.ast.arena[node.children[1]].clone();
        let left_side = self.expect_expr(left_child);
        let right_side = self.expect_expr(right_child);
        
        match (left_side, right_side) {
            (Value::Int(t), Value::Int(v)) => {
                let sub = t - v;
                Value::Int(sub)
            }
            _ => panic!("Incomtabile substract of two values")
        }
    }
    fn handle_multiply(&mut self, node: Node<AstItem>) -> Value {
        let left_child = self.ast.arena[node.children[0]].clone();
        let right_child = self.ast.arena[node.children[1]].clone();
        let left_side = self.expect_expr(left_child);
        let right_side = self.expect_expr(right_child);
        
        match (left_side, right_side) {
            (Value::Int(t), Value::Int(v)) => {
                let mul = t * v;
                Value::Int(mul)
            }
            _ => panic!("Incomtabile multiply of two values")
        }
    }
    fn handle_divide(&mut self, node: Node<AstItem>) -> Value {
        let left_child = self.ast.arena[node.children[0]].clone();
        let right_child = self.ast.arena[node.children[1]].clone();
        let left_side = self.expect_expr(left_child);
        let right_side = self.expect_expr(right_child);
        
        match (left_side, right_side) {
            (Value::Int(t), Value::Int(v)) => {
                let div = t / v;
                Value::Int(div)
            }
            _ => panic!("Incomtabile division of two values")
        }
    }
    fn handle_less_than(&mut self, node: Node<AstItem>) -> Value {
        let left_child = self.ast.arena[node.children[0]].clone();
        let right_child = self.ast.arena[node.children[1]].clone();
        let left_side = self.expect_expr(left_child);
        let right_side = self.expect_expr(right_child);
        
        match (left_side, right_side) {
            (Value::Int(t), Value::Int(v)) => {
                Value::Bool(t < v)
            }
            _ => panic!("Incomtabile comparison of two values")
        }
    }
    fn handle_equal(&mut self, node: Node<AstItem>) -> Value {
        let left_child = self.ast.arena[node.children[0]].clone();
        let right_child = self.ast.arena[node.children[1]].clone();
        let left_side = self.expect_expr(left_child);
        let right_side = self.expect_expr(right_child);
        
        match (left_side, right_side) {
            (Value::Int(t), Value::Int(v)) => {
                Value::Bool(t == v)
            }
            (Value::String(t), Value::String(v)) => {
                Value::Bool(t.eq(&v))
            }
            (Value::Bool(t), Value::Bool(v)) => {
                Value::Bool(t == v)
            }
            _ => panic!("Incomtabile comparison of two values")
        }
    }
    fn handle_and(&mut self, node: Node<AstItem>) -> Value {
        let left_child = self.ast.arena[node.children[0]].clone();
        let right_child = self.ast.arena[node.children[1]].clone();
        let left_side = self.expect_expr(left_child);
        let right_side = self.expect_expr(right_child);
        
        match (left_side, right_side) {
            (Value::Bool(t), Value::Bool(v)) => {
                Value::Bool(t && v)
            }
            _ => panic!("Incomtabile comparison of two values")
        }
    }

    fn expect_opnd(&mut self, node: Node<AstItem>) -> Value {
        match &node.val {
            AstItem::Constant(t) => {
                match t.const_type {
                    VariableType::String => {
                        return Value::String(t.value.clone())
                    }
                    VariableType::Int => {
                        return Value::Int(t.value.clone().parse().unwrap())
                    }
                    VariableType::Bool => {
                        let b_value;
                        match t.value.as_str() {
                            "true" => b_value = true,
                            "false" => b_value = false,
                            _ => panic!("UNEXPECTED ERROR bool value is not true or false")
                        }
                        return Value::Bool(b_value)
                    }
                    
                }
            }
            AstItem::Not => {
                let child_node = self.ast.arena[node.children[0]].clone();
                let opnd = self.expect_opnd(child_node);
                match opnd {
                    Value::Bool(t) => {
                        return Value::Bool(!t)
                    }

                    _ => {
                        panic!("ERROR trying to use logical not on non-boolean value");
                    }
                }
            }
            AstItem::Variable(t) => {
                let value = self.variables.get(&t.name);
                match value {
                    Some(t) => return t.value.clone(),
                    None => panic!("NULL Reference")
                }
            }
            _ => {
                return self.expect_expr(node);
            }
        }
    }
}
fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}

struct RunTimeVariable {
    name: String,
    var_type: VariableType,
    value: Value

}

#[derive(Clone)]
enum Value {
    Int(i32),
    String(String),
    Bool(bool),
    NULL
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(t) => write!(f, "{}", t),
            Value::String(t) => write!(f, "{}", t),
            Value::Bool(t) => write!(f, "{}", t),
            Value::NULL => write!(f, "NULL")
        }
    }
}