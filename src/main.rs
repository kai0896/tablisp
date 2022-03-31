use std::fs;
use std::error::Error;
use std::collections::HashMap;

pub mod funcs;
use funcs::Atom;

fn main() {
    let cells = parse_csv();
    let new_cells = lookup_cell_references(cells);
    println!("new cells: {:?}", new_cells);
}

fn parse_csv() -> Vec<Vec<String>> {
    let contents = fs::read_to_string("../test.csv")
        .expect("Something went wrong reading the file");

    let mut row_vec: Vec<Vec<String>> = Vec::new();

    for line_str in contents.lines() {
        let line_vec = line_str.split(',')
                               .map(|a| a.trim())
                               .map(String::from)
                               .collect::<Vec<String>>();
        row_vec.push(line_vec);
    }
    row_vec
}

fn lookup_cell_references(mut cells: Vec<Vec<String>>) -> Vec<Vec<String>> {
    for i in 0..cells.len(){
        for j in 0..cells[i].len(){
            let cell = cells[i][j].clone();
            if cell.len() > 0 && cell.chars().next().unwrap() == '('{
                cells[i][j] = calc_lisp(&cells, cell)
            }
        }
    }
    cells
}

fn calc_lisp(cells: &Vec<Vec<String>>, sexp: String) -> String {
    let tokens = tokenize(&sexp);
    let size = tokens.len() * 2;
    let tokens_typed: Vec<Atom> = tokens.iter().fold(Vec::with_capacity(size), |mut acc, v| {
        acc.extend(string_to_atom(v.to_string(), &cells)); acc
    });
    match eval(tokens_typed) {
        Ok(a) => a,
        Err(e) => e.to_string()
    }
}

fn tokenize(input: &String) -> Vec<String> {
    input.replace("(", " ( ")
         .replace(")", " ) ")
         .split_whitespace()
         .map(String::from)
         .collect::<Vec<String>>()
}

fn string_to_atom(string: String, cells: &Vec<Vec<String>>) -> Vec<Atom> {
    let atom = match string.as_str() {
        "(" => Atom::Open,
        ")" => Atom::Close,
        _ => {
            let res = string.parse::<f64>();
            match res {
                Ok(fc) => Atom::Number(fc),
                Err(_) => match cell_content_from_refrence(cells, string.clone()) {
                    Some(num) => num,
                    None => Atom::Symbol(string)
                }
            }
        }
    };
    vec!(atom)
}

fn cell_content_from_refrence(cells: &Vec<Vec<String>>, cell_ref: String) -> Option<Atom> {
    // TODO: throw an error when index outside cells shape
    let col_str = cell_ref.chars().nth(0).unwrap();
    let col_int: usize = match col_str {
        'A' => Some(0),
        'B' => Some(1),
        'C' => Some(2),
        'D' => Some(3),
        'E' => Some(4),
        'F' => Some(5),
        'G' => Some(6),
        'H' => Some(7),
        'I' => Some(8),
        _   => None,
    }?;
    let row_str: String = cell_ref.chars().skip(1).collect();
    let row_int = row_str.parse::<usize>().ok()? - 1;

    let res_str = cells.get(row_int)?.get(col_int)?;
    let res_num = res_str.parse::<f64>().ok()?;
    Some(Atom::Number(res_num))
}

fn eval(tokens: Vec<Atom>) -> Result<String, Box<dyn Error>>{
    let mut ops: HashMap<String, fn(Vec<Atom>) -> Result<Atom, Box<dyn Error>>> = HashMap::new();
    ops.insert("+".to_string(), funcs::plus);
    ops.insert("-".to_string(), funcs::minus);
    ops.insert("*".to_string(), funcs::multiply);
    ops.insert(">".to_string(), funcs::greater);
    ops.insert("<".to_string(), funcs::smaller);
    ops.insert("if".to_string(), funcs::if_branch);
    let mut stack = Vec::new();

    for token in &tokens{
        // println!("sta: {:?}", stack);
        match token {
            Atom::Close => {
                let mut sub_stack = Vec::new();

                loop {
                    let atom = stack.pop();
                    match atom {
                        Some(a) => {
                            match a{
                                Atom::Open => break,
                                _ => sub_stack.push(a)
                            }
                        },
                        None => Err("Syntax Error: Missing opening parenthese")?
                    }
                }
                if sub_stack.len() < 2 {
                    Err("Syntax Error: Not enough elements in expression")?
                }
                // println!("sub: {:?}", sub_stack);
                let op = sub_stack.pop().unwrap();
                match op {
                    Atom::Symbol(s) => {
                        match ops.get(&s){
                            Some(val_op) => {
                                match val_op(sub_stack) {
                                    Ok(res) => stack.push(res),
                                    Err(e) => Err(e)?
                                }
                            },
                            None => Err(format!("Syntax Error: func '{}' not supported", s))?
                        }
                    },
                    _ => Err("Syntax Error: first element has to be a function")?
                }
            }
            _ => stack.push(token.clone())
        }
    }
    if stack.len() == 1 {
        match stack[0] {
            Atom::Number(num) => {
                if num.fract() == 0.0 {
                    return Ok((num as i32).to_string())
                } else {
                    return Ok(num.to_string())
                }
            },
            _ => Err(format!("Syntax Error: result '{:?}' is not a number", stack[0]))?
        }
    } else {
        Err("Syntax Error: Missing closing parenthese")?
    }
}

