use std::error::Error;

// #[derive(Clone, Debug)]
// pub enum Atom {
//     Open,
//     Close,
//     Number(f64),
//     Symbol(String),
//     True,
//     False,
// }

// pub fn plus(tokens: Vec<Atom>) -> Result<Atom, Box<dyn Error>> {
//     let res: Result<f64, _> = tokens.iter().try_fold(0.0, |acc, a| match a {
//         Atom::Number(num) => Ok(acc + num),
//         _ => Err("Syntax Error: + can only be used with numbers")
//     });
//     Ok(Atom::Number(res?))
// }

// pub fn multiply(tokens: Vec<Atom>) -> Result<Atom, Box<dyn Error>> {
//     let res: Result<f64, _> = tokens.iter().try_fold(1.0, |acc, a| match a {
//         Atom::Number(num) => Ok(acc * num),
//         _ => Err("Syntax Error: + can only be used with numbers")
//     });
//     Ok(Atom::Number(res?))
// }

// pub fn minus(mut tokens: Vec<Atom>) -> Result<Atom, Box<dyn Error>> {
//     let first = tokens.pop().unwrap();
//     match first {
//         Atom::Number(first_num) => {
//             let res: Result<f64, Box<dyn Error>> = tokens.iter().try_fold(first_num, |acc, a| match a {
//                 Atom::Number(num) => Ok(acc - num),
//                 _ => Err("Syntax Error: - can only be used with numbers")?
//             });
//             Ok(Atom::Number(res?))
//         },
//         _ => Err("Syntax Error: - can only be used with numbers")?
//     }
// }

pub fn greater(tokens: Vec<Atom>) -> Result<Atom, Box<dyn Error>> {
    if tokens.len() == 2 {
        let tokens_num: Result<Vec<&f64>, Box<dyn Error>> = tokens.iter().map(|a| match a {
            Atom::Number(num) => return Ok(num),
            _ => Err("Syntax Error: > can only be used with numbers")?
        }).collect();
        let nums = tokens_num?;
        if nums[1] > nums[0] {
            return Ok(Atom::True)
        } else {
            return Ok(Atom::False)
        }
    } else {
        Err("Syntax Error: > requires exactly 2 arguments")?
    }
}

pub fn smaller(tokens: Vec<Atom>) -> Result<Atom, Box<dyn Error>> {
    if tokens.len() == 2 {
        let tokens_num: Result<Vec<&f64>, Box<dyn Error>> = tokens.iter().map(|a| match a {
            Atom::Number(num) => return Ok(num),
            _ => Err("Syntax Error: < can only be used with numbers")?
        }).collect();
        let nums = tokens_num?;
        if nums[1] < nums[0] {
            return Ok(Atom::True)
        } else {
            return Ok(Atom::False)
        }
    } else {
        Err("Syntax Error: < requires exactly 2 arguments")?
    }
}

pub fn if_branch(mut tokens: Vec<Atom>) -> Result<Atom, Box<dyn Error>> {
    if tokens.len() == 3 {
        let first = tokens.pop().unwrap();
        match first {
            Atom::True => Ok(tokens[1].clone()),
            Atom::False => Ok(tokens[0].clone()),
            _ => Err("Syntax Error: first element of 'if' has to be a boolean")?
        }

    } else {
        Err("Syntax Error: 'if' requires exactly 3 arguments")?
    }
}

pub fn max_num(mut tokens: Vec<Atom>) -> Result<Atom, Box<dyn Error>> {
    tokens.iter().filter_map(|a| match a {
        Atom::Number(num) => num,
        _ => None
    })
}
