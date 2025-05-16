use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(Debug, Clone)]
enum Value {
    Int(i32),
    Str(String),
    Bool(bool),
}

impl Value {
    // Convert Value to bool for conditions
    fn as_bool(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::Int(n) => *n != 0,
            Value::Str(s) => !s.is_empty(),
        }
    }

    // Convert Value to int or panic
    fn as_int(&self) -> i32 {
        match self {
            Value::Int(n) => *n,
            _ => panic!("Expected integer value"),
        }
    }
}

fn split(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_string = false;

    for c in input.chars() {
        if c == '"' {
            in_string = !in_string;
            current.push(c);
            if !in_string {
                tokens.push(current.clone());
                current.clear();
            }
        } else if in_string {
            current.push(c);
        } else if c.is_whitespace() {
            if !current.is_empty() {
                tokens.push(current.clone());
                current.clear();
            }
        } else {
            current.push(c);
        }
    }
    if !current.is_empty() {
        tokens.push(current);
    }
    tokens
}

fn main() {
    let filename = env::args().nth(1).expect("Usage: rorthi <filename>");
    let content = fs::read_to_string(filename).expect("Failed to read file");
    let tokens = split(&content);
    
    let mut stack: Vec<Value> = Vec::new();
    let _string_stack: Vec<String> = Vec::new(); // optional, could merge with stack
    let mut memory: HashMap<String, Value> = HashMap::new();

    let mut pc: usize = 0;
    let mut loop_stack: Vec<usize> = Vec::new();
    let mut begin_stack: Vec<usize> = Vec::new();

    while pc < tokens.len() {
        match tokens[pc].as_str() {
            "+" => {
                let b = stack.pop().expect("Stack underflow").as_int();
                let a = stack.pop().expect("Stack underflow").as_int();
                stack.push(Value::Int(a + b));
            }
            "-" => {
                let b = stack.pop().expect("Stack underflow").as_int();
                let a = stack.pop().expect("Stack underflow").as_int();
                stack.push(Value::Int(a - b));
            }
            "*" => {
                let b = stack.pop().expect("Stack underflow").as_int();
                let a = stack.pop().expect("Stack underflow").as_int();
                stack.push(Value::Int(a * b));
            }
            "/" => {
                let b = stack.pop().expect("Stack underflow").as_int();
                let a = stack.pop().expect("Stack underflow").as_int();
                stack.push(Value::Int(a / b));
            }
            "^" => {
                let b = stack.pop().expect("Stack underflow").as_int();
                let a = stack.pop().expect("Stack underflow").as_int();
                stack.push(Value::Int(a.pow(b as u32)));
            }
            "." => {
                if let Some(val) = stack.last() {
                    match val {
                        Value::Int(n) => println!("{}", n),
                        Value::Bool(b) => println!("{}", b),
                        Value::Str(s) => println!("{}", s),
                    }
                } else {
                    println!("Stack is empty");
                }
            }
            "dup" => {
                let top = stack.last().expect("Stack underflow for dup").clone();
                stack.push(top);
            }
            "while" => {
                loop_stack.push(pc);
            }
            "if" => {
                let condition = stack.pop().expect("Missing condition for if");
                if !condition.as_bool() {
                    // Skip to matching 'end'
                    let mut depth = 1;
                    while depth > 0 {
                        pc += 1;
                        if pc >= tokens.len() {
                            panic!("Unmatched if");
                        }
                        if tokens[pc] == "if" {
                            depth += 1;
                        } else if tokens[pc] == "end" {
                            depth -= 1;
                        }
                    }
                }
            }
            "end" => {
                if let Some(pos) = loop_stack.last() {
                    pc = *pos - 1;
                }
            }
            "mem" => {
                let name = tokens.get(pc + 1).expect("Expected name after mem");
                memory.insert(name.clone(), Value::Int(0));
                pc += 1;
            }
            "store" => {
                let name = tokens.get(pc + 1).expect("Expected name after store");
                let val = stack.pop().expect("Stack underflow");
                memory.insert(name.clone(), val);
                pc += 1;
            }
            "load" => {
                let name = tokens.get(pc + 1).expect("Expected name after load");
                match memory.get(name) {
                    Some(val) => stack.push(val.clone()),
                    None => panic!("Variable {} not found", name),
                }
                pc += 1;
            }
            "begin" => {
                begin_stack.push(pc);
            }
            "repeat" => {
                let condition = stack.pop().expect("Missing condition for repeat");
                if condition.as_bool() {
                    if let Some(&pos) = begin_stack.last() {
                        pc = pos;
                    } else {
                        panic!("repeat without begin");
                    }
                } else {
                    begin_stack.pop();
                }
            }
            token => {
                if token == "true" {
                    stack.push(Value::Bool(true));
                } else if token == "false" {
                    stack.push(Value::Bool(false));
                } else if token.starts_with('"') && token.ends_with('"') {
                    stack.push(Value::Str(token[1..token.len()-1].to_string()));
                } else if let Ok(n) = token.parse::<i32>() {
                    stack.push(Value::Int(n));
                } else {
                    panic!("Unknown token: {}", token);
                }
            }
        }
        pc += 1;
    }
}
