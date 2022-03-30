use std::fs;
use std::collections::HashMap;
use std::error::Error;

type BoxResult<T> = Result<T,Box<dyn Error>>;

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

fn eval_lisp(cells: &Vec<Vec<String>>, sexp: String) -> String {

    cell_content_from_refrence(&cells, sexp.replace("(", "").replace(")", ""))
}

fn eval_cell_references(mut cells: Vec<Vec<String>>) -> Vec<Vec<String>> {
    for i in 0..cells.len(){
        for j in 0..cells[i].len(){
            let cell = cells[i][j].clone();
            if cell.len() > 0 && cell.chars().next().unwrap() == '('{
                cells[i][j] = eval_lisp(&cells, cell)
            }
        }
    }
    cells
}

type Symbol = String;

#[derive(Debug)]
enum Number {
    IntNum(i32),
    FloatNum(f64)
}

#[derive(Debug)]
enum Atom {
    Symbol(Symbol),
    Number(Number)
}

#[derive(Debug)]
enum Exp {
    Atom(Atom),
    List(Vec<Exp>)
}

fn read_from_tokens(tokens: Vec<String>, mut pointer: usize) -> (Exp, usize) {
    let token = tokens[pointer].clone();
    pointer += 1;
    if token == "(" {
        let mut l_res = Vec::new();
        while tokens[pointer] != ")" {
            let (exp, p) = read_from_tokens(tokens.clone(), pointer);
            l_res.push(exp);
            pointer = p;
        }
        (Exp::List(l_res), pointer+1)
    } else {
        let res = token.parse::<i32>();
        match res {
            Ok(fc) => (Exp::Atom(Atom::Number(Number::IntNum(fc))), pointer),
            Err(_) => {
                let res = token.parse::<f64>();
                match res {
                    Ok(fc) => (Exp::Atom(Atom::Number(Number::FloatNum(fc))), pointer),
                    Err(_) => (Exp::Atom(Atom::Symbol(token)), pointer)
                }
            }
        }
    }
}

// fn travers_ast(list_item: Exp) -> BoxResult<Exp> {
//     match list_item {
//         Exp::Atom(ref res) => match res {
//             Atom::Number(_) => return Ok(list_item),
//             Atom::Symbol(_) => return Ok(list_item)
//         }
//         Exp::List(ref list) => {
//             match list[0] {
//                 Exp::List(_) => Err("wrong"),
//                 Exp::Atom(atom) => {
//                     match atom {
//                         Atom::Symbol(symb) => {
//                             let args: Vec<i32> = Vec::new();
//                             for arg in list[1..2].iter(){
//                                 let new_arg = travers_ast(arg).unwrap();
//                                 match new_arg {
//                                     Exp::List(_) => panic!("syntax error"),
//                                     Exp::Atom(atom) => {
//                                         match atom {
//                                             Atom::Symbol(_) => panic!("syntax error"),
//                                             Atom::Number(num) => {
//                                                 match num {
//                                                     Number::FloatNum(_) => panic!("syntax error"),
//                                                     Number::IntNum(int_n) => args.push(int_n)
//                                                 }
//                                             }
//                                         }
//                                     }
//                                 }
//                             }
//                             match symb.as_str() {
//                                 "+" => {
//                                     return Ok(Exp::Atom(Atom::Number(Number::IntNum(args[0] + args[1]))));
//                                 }
//                                 _ => panic!("syntax error"),                            }
//                         }
//                         Atom::Number(_) => panic!("syntax error"),
//                     }
//                 }
//         }
//     }
// }

fn eval(list_item: Exp) -> Atom {
    match list_item {
        Exp::Atom(a) => return a,
        Exp::List(list) => {
            let stack: Vec<Atom> = Vec::new();
            for item in list.iter(){
                match item {
                    Exp::Atom(a) => stack.push(a),
                    Exp::List(list) {


                    }
                };
            };
        }
    }

}

fn tokenize(input: &String) -> Vec<String> {
    input.replace("(", " ( ")
         .replace(")", " ) ")
         .split_whitespace()
         .map(String::from)
         .collect::<Vec<String>>()
}

fn main() {
    // let cells = parse_csv();
    // let new_cells = eval_cell_references(cells);
    // println!("new cells: {:?}", new_cells);

    // let item = Exp::List(Vec::from([Exp::Atom(String::from("hallo")), Exp::Atom(String::from("no"))]));
    // travers_ast(&item);
    let tests = String::from("(+ 5 (+ (+ 0.5 1) 2))");
    // travers_ast(&read_from_tokens(tokenize(&tests), 0));
    println!("{:?}", read_from_tokens(tokenize(&tests), 0));
}
