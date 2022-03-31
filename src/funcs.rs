use std::error::Error;

#[derive(Clone, Debug)]
pub enum Atom {
    Open,
    Close,
    Number(f64),
    Symbol(String),
    True,
    False,
}

pub fn plus(tokens: Vec<Atom>) -> Result<Atom, Box<dyn Error>> {
    let res: Result<f64, _> = tokens.iter().try_fold(0.0, |acc, a| match a {
        Atom::Number(num) => Ok(acc + num),
        _ => Err("Syntax Error: + can only be used with numbers")
    });
    match res {
        Ok(val) => Ok(Atom::Number(val)),
        Err(e) => Err(e)?
    }
}

pub fn multiply(tokens: Vec<Atom>) -> Result<Atom, Box<dyn Error>> {
    let res: Result<f64, _> = tokens.iter().try_fold(1.0, |acc, a| match a {
        Atom::Number(num) => Ok(acc * num),
        _ => Err("Syntax Error: + can only be used with numbers")
    });
    match res {
        Ok(val) => Ok(Atom::Number(val)),
        Err(e) => Err(e)?
    }
}

pub fn minus(mut tokens: Vec<Atom>) -> Result<Atom, Box<dyn Error>> {
    let first = tokens.pop().unwrap();
    match first {
        Atom::Number(first_num) => {
            let res: Result<f64, _> = tokens.iter().try_fold(first_num, |acc, a| match a {
                Atom::Number(num) => Ok(acc - num),
                _ => Err("Syntax Error: - can only be used with numbers")?
            });
            match res {
                Ok(val) => Ok(Atom::Number(val)),
                Err(e) => Err(e)
            }
        },
        _ => Err("Syntax Error: - can only be used with numbers")?
    }
}

pub fn greater(tokens: Vec<Atom>) -> Result<Atom, Box<dyn Error>> {
    if tokens.len() == 2 {
            let tokens_num: Result<Vec<_>, _> = tokens.iter().map(|a| match a {
                Atom::Number(num) => return Ok(num),
                _ => Err("Syntax Error: > can only be used with numbers")?
            }).collect();
        match tokens_num {
            Ok(nums) => {
                if nums[1] > nums[0] {
                    return Ok(Atom::True)
                } else {
                    return Ok(Atom::False)
                }
            }
            Err(e) => Err(e)
        }
    } else {
        Err("Syntax Error: > requires exactly 2 arguments")?
    }
}

pub fn smaller(tokens: Vec<Atom>) -> Result<Atom, Box<dyn Error>> {
    if tokens.len() == 2 {
            let tokens_num: Result<Vec<_>, _> = tokens.iter().map(|a| match a {
                Atom::Number(num) => return Ok(num),
                _ => Err("Syntax Error: > can only be used with numbers")?
            }).collect();
        match tokens_num {
            Ok(nums) => {
                if nums[1] < nums[0] {
                    return Ok(Atom::True)
                } else {
                    return Ok(Atom::False)
                }
            }
            Err(e) => Err(e)
        }
    } else {
        Err("Syntax Error: > requires exactly 2 arguments")?
    }
}
