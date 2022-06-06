use std::error::Error;
use std::collections::HashMap;

pub mod funcs;
use funcs::Atom;

#[derive(PartialEq)]
pub enum CellType {
    Number,
    String,
    Error
}

pub struct Cell {
    pub content: String,
    pub result: Option<String>,
    pub cell_type: CellType,
}

pub fn compute_cells(cells: &mut Vec<Vec<Cell>>){
    for i in 0..cells.len(){
        for j in 0..cells[i].len(){
            let cell = cells[i][j].content.clone();
            if cell.len() > 0 && cell.chars().next().unwrap() == '('{
                match calc_lisp(&cells, cell) {
                    Ok(str) => {
                        cells[i][j].result = Some(str);
                        cells[i][j].cell_type = CellType::Number;
                    },
                    Err(err) => {
                        cells[i][j].result = Some(err.to_string());
                        cells[i][j].cell_type = CellType::Error;
                    }
                }
            }
        }
    }
}

pub fn get_cell_type(string: &String) -> CellType {
    match string.parse::<f64>() {
        Ok(_) => CellType::Number,
        Err(_) => CellType::String
    }
}

fn calc_lisp(cells: &Vec<Vec<Cell>>, sexp: String) -> Result<String, Box<dyn Error>> {
    let tokens = tokenize(&sexp);
    let size = tokens.len() * 2;
    let tokens_typed: Vec<Atom> = tokens.iter().fold(Vec::with_capacity(size), |mut acc, v| {
        acc.extend(string_to_atom(v.to_string(), &cells)); acc
    });
    eval(tokens_typed)
}

fn tokenize(input: &String) -> Vec<String> {
    input.replace("(", " ( ")
         .replace(")", " ) ")
         .split_whitespace()
         .map(String::from)
         .collect::<Vec<String>>()
}

fn string_to_atom(string: String, cells: &Vec<Vec<Cell>>) -> Vec<Atom> {
    match string.as_str() {
        "(" => vec!(Atom::Open),
        ")" => vec!(Atom::Close),
        _ => {
            let res = string.parse::<f64>();
            match res {
                Ok(fc) => vec!(Atom::Number(fc)),
                Err(_) => match cell_content_from_refrence(cells, string.clone()) {
                    Some(num) => vec!(num),
                    None => match cell_content_from_range(cells, string.clone()) {
                        Some(nums) => nums,
                        None => vec!(Atom::Symbol(string))
                    }
                }
            }
        }
    }
}

fn cell_content_from_range(cells: &Vec<Vec<Cell>>, cell_range: String) -> Option<Vec<Atom>> {
    let cell_refs = cell_range.split(":").map(String::from).collect::<Vec<String>>();

    if cell_refs.len() == 2 {
        let range_begin = get_cell_coordinates(cell_refs[0].clone())?;
        let range_end = get_cell_coordinates(cell_refs[1].clone())?;

        let mut res: Vec<Atom> = Vec::new(); // maybe init with size
        for row in range_begin.0..=range_end.0 {
            for col in range_begin.1..=range_end.1 {
                match cells.get(row)?.get(col) {
                    Some(cell) => {
                        let text = match &cell.result {
                            Some(str) => str.clone(),
                            None => cell.content.clone()
                        };
                        match text.parse::<f64>() {
                            Ok(num) => res.push(Atom::Number(num)),
                            Err(_) => ()
                        }
                    }
                    None => ()
                }
            }
        }
        Some(res)

    } else {
        None
    }
}

fn cell_content_from_refrence(cells: &Vec<Vec<Cell>>, cell_ref: String) -> Option<Atom> {
    let coords = get_cell_coordinates(cell_ref)?;

    let cell = &cells.get(coords.0)?.get(coords.1)?;
    let res_str = match &cell.result {
        Some(str) => str.clone(),
        None => cell.content.clone()
    };
    let res_num = res_str.parse::<f64>().ok()?;
    Some(Atom::Number(res_num))
}

fn get_cell_coordinates(cell_ref: String) -> Option<(usize, usize)> {
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
    Some((row_int, col_int))
}

fn eval(tokens: Vec<Atom>) -> Result<String, Box<dyn Error>>{
    let mut ops: HashMap<String, fn(Vec<Atom>) -> Result<Atom, Box<dyn Error>>> = HashMap::new();
    ops.insert("+".to_string(), funcs::plus);
    ops.insert("-".to_string(), funcs::minus);
    ops.insert("*".to_string(), funcs::multiply);
    ops.insert("/".to_string(), funcs::divide);
    ops.insert(">".to_string(), funcs::greater);
    ops.insert("<".to_string(), funcs::smaller);
    ops.insert("if".to_string(), funcs::if_branch);
    ops.insert("max".to_string(), funcs::max_num);
    ops.insert("min".to_string(), funcs::min_num);
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
                                let res = val_op(sub_stack)?;
                                stack.push(res);
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

#[cfg(test)]
mod tests {
    use super::*;

}
