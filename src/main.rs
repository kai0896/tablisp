use std::fs;
use std::error::Error;

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

fn cell_content_from_refrence(cells: &Vec<Vec<String>>, cell_ref: String) -> String {
    // TODO: throw an error when index outside cells shape
    let row_str: String = cell_ref.chars().skip(1).collect();
    let row_int = row_str.parse::<usize>().unwrap() -1;
    let col_str = cell_ref.chars().nth(0).unwrap();
    let col_int: usize = match col_str {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        'D' => 3,
        'E' => 4,
        'F' => 5,
        'G' => 6,
        'H' => 7,
        'I' => 8,
        _   => 0,
    };
    cells[row_int][col_int].clone()
}

fn calc_lisp(cells: &Vec<Vec<String>>, sexp: String) -> String {

    cell_content_from_refrence(&cells, sexp.replace("(", "").replace(")", ""))
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

#[derive(Clone, Debug)]
enum Atom {
    Open,
    Close,
    Number(f64),
    Symbol(String)
}

fn tokenize(input: &String) -> Vec<String> {
    input.replace("(", " ( ")
         .replace(")", " ) ")
         .split_whitespace()
         .map(String::from)
         .collect::<Vec<String>>()
}

fn string_to_atom(string: String) -> Vec<Atom> {
    let atom = match string.as_str() {
        "(" => Atom::Open,
        ")" => Atom::Close,
        _ => {
            let res = string.parse::<f64>();
            match res {
                Ok(fc) => Atom::Number(fc),
                Err(_) => Atom::Symbol(string)
            }
        }
    };
    vec!(atom)
}

fn read_from_tokens(tokens: Vec<String>) -> Vec<Atom> {
    let size = tokens.len() * 2;
    tokens.iter().fold(Vec::with_capacity(size), |mut acc, v| {
        acc.extend(string_to_atom(v.to_string())); acc
    })
}

fn eval(tokens: Vec<Atom>) -> Result<String, Box<dyn Error>>{
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
                        if s == "+" {
                            let res: Result<f64, _> = sub_stack.iter().try_fold(0.0, |acc, a| match a {
                                Atom::Number(num) => Ok(acc + num),
                                _ => Err("Syntax Error: + can only be used with numbers")
                            });
                            match res {
                                Ok(val) => stack.push(Atom::Number(val)),
                                Err(e) => Err(e)?
                            }
                        }
                    },
                    _ => ()
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
            _ => Err("Syntax Error: result is not a number")?
        }
    } else {
        Err("Syntax Error: Missing closing parenthese")?
    }
}

fn main() {
    // let cells = parse_csv();
    // let new_cells = eval_cell_references(cells);
    // println!("new cells: {:?}", new_cells);

    // let item = Exp::List(Vec::from([Exp::Atom(String::from("hallo")), Exp::Atom(String::from("no"))]));
    // travers_ast(&item);
    let tests = String::from("(+ 5 (+ (+ 3 9.1) (+ 3 3 3)))");
    // travers_ast(&read_from_tokens(tokenize(&tests), 0));
    let tokens = read_from_tokens(tokenize(&tests));
    println!("{:?}", tokens);
    let res = eval(tokens);
    println!("{:?}", res);
}
