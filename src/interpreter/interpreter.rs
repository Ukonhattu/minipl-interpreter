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
            ast,
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
        let variable_node = self.ast.arena[node.children[0]].clone();
        let range_node = self.ast.arena[node.children[1]].clone();
        let block_node = self.ast.arena[node.children[2]].clone();
        let variable_info;
        let mut variable;
        if let AstItem::Variable(t) = variable_node.clone().val {
            variable_info = t;
        } else {
            panic!("ERROR for statement expected variable, found {:#?}", variable_node.val);
        }
        let variable_option = self.variables.get(&variable_info.name);
        match variable_option {
            Some(t) => variable = t.clone(),
            None => panic!("Undefined variable {:#?}", variable_node.val)
        }
        let range_left_node = self.ast.arena[range_node.children[0]].clone();
        let range_right_node = self.ast.arena[range_node.children[1]].clone();
        let range_left_expr = self.expect_expr(range_left_node);
        let range_right_expr = self.expect_expr(range_right_node);
        let range_left;
        let range_right;
        match (range_left_expr.clone(), range_right_expr.clone()) {
            (Value::Int(t), Value::Int(v)) => {
                range_left = t;
                range_right = v;
            }
            _ => {
                panic!("ERROR range values are not int. Left = {}, Right = {}", range_left_expr, range_right_expr)
            }
        }

        variable.value = Value::Int(range_left);
        self.variables.insert(variable.name.clone(), variable.clone());
        for _ in range_left..=range_right {
            let block_children = block_node.children.clone();
            for child in block_children {
                let node = self.ast.arena[child].clone();
                self.parse_node(node);
            }
            let old_value;
            match variable.value {
                Value::Int(t) => {
                    old_value = t;
                }
                _ => {
                    panic!("Loop variable is not int {:#?}", node);
                }
            }
            variable.value = Value::Int(old_value + 1);
            self.variables.insert(variable.name.clone(), variable.clone());
        }
        
    }

    fn handle_assert(&mut self, node: Node<AstItem>) {
        let child = self.ast.arena[node.children[0]].clone();
        let expr = self.expect_expr(child);
        match expr {
            Value::Bool(t) => {
                if t {
                    return; 
                } else {
                    panic!("ASSERT FAILED")
                }
            }
            _ => {
                panic!("ERROR Assert expression is not evaluated into a bool");
            }
        }
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
                var = RunTimeVariable{name: var_name.clone(), var_type, value: Value::Int(line.trim().parse().unwrap())}
            }
            VariableType::String => {
                trim_newline(&mut line);
                var = RunTimeVariable{name: var_name.clone(), var_type, value: Value::String(line)}
            }
            VariableType::Bool => panic!("Cannot read a boolean value")
        }
        if let std::collections::hash_map::Entry::Occupied(mut e) = self.variables.entry(var_name) {
            e.insert(var);
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
            var = RunTimeVariable {name: var_name.clone(), var_type, value: var_value};    
        } else {
            var = RunTimeVariable {name: var_name.clone(), var_type, value: Value::NULL};
        }
        self.variables.insert(var_name, var);
    }

    fn expect_expr(&self, node: Node<AstItem>) -> Value {
        if node.children.is_empty() {
            return self.expect_opnd(node)
        }
        match node.val.clone() {
            AstItem::BinOp(t) => {
                match t {
                    BinOpType::And => self.handle_and(node),
                    BinOpType::Divide => self.handle_divide(node),
                    BinOpType::Equal => self.handle_equal(node),
                    BinOpType::LessThan => self.handle_less_than(node),
                    BinOpType::Minus => self.handle_minus(node),
                    BinOpType::Multiply => self.handle_multiply(node),  
                    BinOpType::Plus => self.handle_plus(node),           
                }
            }
            _ => panic!("Error, unexpected node {:#?}", node)
        }
    }
    
    fn handle_plus(&self, node: Node<AstItem>) -> Value {
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
    fn handle_minus(&self, node: Node<AstItem>) -> Value {
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
    fn handle_multiply(&self, node: Node<AstItem>) -> Value {
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
    fn handle_divide(&self, node: Node<AstItem>) -> Value {
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
    fn handle_less_than(&self, node: Node<AstItem>) -> Value {
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
    fn handle_equal(&self, node: Node<AstItem>) -> Value {
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
    fn handle_and(&self, node: Node<AstItem>) -> Value {
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

    fn expect_opnd(&self, node: Node<AstItem>) -> Value {
        match &node.val {
            AstItem::Constant(t) => {
                match t.const_type {
                    VariableType::String => {
                        Value::String(t.value.clone())
                    }
                    VariableType::Int => {
                        Value::Int(t.value.clone().parse().unwrap())
                    }
                    VariableType::Bool => {
                        let b_value;
                        match t.value.as_str() {
                            "true" => b_value = true,
                            "false" => b_value = false,
                            _ => panic!("UNEXPECTED ERROR bool value is not true or false")
                        }
                        Value::Bool(b_value)
                    }
                    
                }
            }
            AstItem::Not => {
                let child_node = self.ast.arena[node.children[0]].clone();
                let opnd = self.expect_opnd(child_node);
                match opnd {
                    Value::Bool(t) => {
                        Value::Bool(!t)
                    }

                    _ => {
                        panic!("ERROR trying to use logical not on non-boolean value");
                    }
                }
            }
            AstItem::Variable(t) => {
                let value = self.variables.get(&t.name);
                match value {
                    Some(t) => t.value.clone(),
                    None => panic!("NULL Reference")
                }
            }
            _ => {
                self.expect_expr(node)
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

#[derive(Clone)]
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