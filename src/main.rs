use std::fs;
use std::convert::TryInto;
use std::env;

fn split(input: &str, delimiter: Option<&str>) -> Vec<String> {
    match delimiter {
        Some(d) => input.split(d).map(|s| s.to_string()).collect(),
        None => input.split_whitespace().map(|s| s.to_string()).collect(),
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() > 2 || args.len() == 1 {
        eprintln!("Error: Expected one argument.");
        std::process::exit(1);
    }

    let filename = &args[1];
    let content = fs::read_to_string(filename).expect("REASON");
    let mut no_whitespace: Vec<String> = split(content.as_str(), None);
    for (index, token) in no_whitespace.clone().iter().enumerate() {
        if token == "+" {
            let a = no_whitespace[index - 2].clone().parse::<i32>().unwrap();
            let b = no_whitespace[index - 1].clone().parse::<i32>().unwrap();
            no_whitespace[index] = (a + b).to_string();
        }
        if token == "-" {
            let a = no_whitespace[index - 2].clone().parse::<i32>().unwrap();
            let b = no_whitespace[index - 1].clone().parse::<i32>().unwrap();
            no_whitespace[index] = (a - b).to_string();
        }
        if token == "*" {
            let a = no_whitespace[index - 2].clone().parse::<i32>().unwrap();
            let b = no_whitespace[index - 1].clone().parse::<i32>().unwrap();
            no_whitespace[index] = (a * b).to_string();
        }
        if token == "/" {
            let a = no_whitespace[index - 2].clone().parse::<i32>().unwrap();
            let b = no_whitespace[index - 1].clone().parse::<i32>().unwrap();
            no_whitespace[index] = (a / b).to_string();
        }
        if token == "^" {
            let a = no_whitespace[index - 2].clone().parse::<i32>().unwrap();
            let b = no_whitespace[index - 1].clone().parse::<i32>().unwrap();
            no_whitespace[index] = a.pow(b.try_into().unwrap()).to_string();
        }

        if token == "." {
            println!("{}", no_whitespace[index - 1]);
        }

    }
}
