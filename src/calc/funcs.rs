use std::error::Error;

#[derive(Clone, Debug, PartialEq)]
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
    Ok(Atom::Number(res?))
}

pub fn multiply(tokens: Vec<Atom>) -> Result<Atom, Box<dyn Error>> {
    let res: Result<f64, _> = tokens.iter().try_fold(1.0, |acc, a| match a {
        Atom::Number(num) => Ok(acc * num),
        _ => Err("Syntax Error: * can only be used with numbers")
    });
    Ok(Atom::Number(res?))
}

pub fn minus(mut tokens: Vec<Atom>) -> Result<Atom, Box<dyn Error>> {
    let first = tokens.pop().unwrap();
    match first {
        Atom::Number(first_num) => {
            let res: Result<f64, Box<dyn Error>> = tokens.iter().try_fold(first_num, |acc, a| match a {
                Atom::Number(num) => Ok(acc - num),
                _ => Err("Syntax Error: - can only be used with numbers")?
            });
            Ok(Atom::Number(res?))
        },
        _ => Err("Syntax Error: - can only be used with numbers")?
    }
}

pub fn divide(mut tokens: Vec<Atom>) -> Result<Atom, Box<dyn Error>> {
    let first = tokens.pop().unwrap();
    match first {
        Atom::Number(first_num) => {
            let res: Result<f64, Box<dyn Error>> = tokens.iter().try_fold(first_num, |acc, a| match a {
                Atom::Number(num) => Ok(acc / num),
                _ => Err("Syntax Error: / can only be used with numbers")?
            });
            Ok(Atom::Number(res?))
        },
        _ => Err("Syntax Error: / can only be used with numbers")?
    }
}
pub fn equal(tokens: Vec<Atom>) -> Result<Atom, Box<dyn Error>> {
    if tokens.len() == 2 {
        let tokens_num: Result<Vec<&f64>, Box<dyn Error>> = tokens.iter().map(|a| match a {
            Atom::Number(num) => return Ok(num),
            _ => Err("Syntax Error: = can only be used with numbers")?
        }).collect();
        let nums = tokens_num?;
        if nums[1] == nums[0] {
            return Ok(Atom::True)
        } else {
            return Ok(Atom::False)
        }
    } else {
        Err("Syntax Error: > requires exactly 2 arguments")?
    }
}

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

pub fn if_branch(tokens: Vec<Atom>) -> Result<Atom, Box<dyn Error>> {
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

pub fn max_num(tokens: Vec<Atom>) -> Result<Atom, Box<dyn Error>> {
    let res = tokens.into_iter().filter_map(|a| match a {
        Atom::Number(num) => Some(num),
        _ => None
    }).collect::<Vec<f64>>();

    let max = res.into_iter()
                 .reduce(f64::max);
    Ok(Atom::Number(max.ok_or("Error: No number given to 'max'")?))
}

pub fn min_num(tokens: Vec<Atom>) -> Result<Atom, Box<dyn Error>> {
    let res = tokens.into_iter().filter_map(|a| match a {
        Atom::Number(num) => Some(num),
        _ => None
    }).collect::<Vec<f64>>();

    let max = res.into_iter()
                 .reduce(f64::min);
    Ok(Atom::Number(max.ok_or("Error: No number given to 'min'")?))
}

#[cfg(test)]
mod tests {
    use super::*;

    // Setup input data: (remember that the order is reversed, because it got pushed into a stack)
    fn get_input_num_and_sym() -> Vec<Atom> {
        vec![Atom::Number(2f64),
             Atom::Number(2.5f64),
             Atom::Symbol("test".to_string())]
    }

    fn get_input_num_only() -> Vec<Atom> {
        vec![Atom::Number(2f64),
             Atom::Number(5f64)]
    }

    // Tests:
    // TODO: test errors
    #[test]
    fn plus_test() {
        assert_eq!(Atom::Number(7f64),
                   plus(get_input_num_only()).unwrap());
    }

   #[test]
    fn multiply_test() {
        assert_eq!(Atom::Number(10f64),
                   multiply(get_input_num_only()).unwrap());
    }

    #[test]
    fn minus_test() {
        assert_eq!(Atom::Number(3f64),
                   minus(get_input_num_only()).unwrap());
    }

    #[test]
    fn divide_test() {
        assert_eq!(Atom::Number(2.5f64),
                   divide(get_input_num_only()).unwrap());
    }

    #[test]
    fn equal_test() {
        assert_eq!(Atom::False,
                   equal(get_input_num_only()).unwrap());

        let input = vec![Atom::Number(2.5f64),
                         Atom::Number(2.5f64),];
        assert_eq!(Atom::True,
                   equal(input).unwrap());
    }

    #[test]
    fn greater_test() {
        assert_eq!(Atom::True,
                   greater(get_input_num_only()).unwrap());
    }

    #[test]
    fn smaller_test() {
        assert_eq!(Atom::False,
                   smaller(get_input_num_only()).unwrap());
    }

    #[test]
    fn if_branch_test() {
        let input = vec![Atom::Number(2.5f64),
                         Atom::Number(5f64),
                         Atom::True,];
        assert_eq!(Atom::Number(5f64),
                   if_branch(input).unwrap());
    }

    #[test]
    fn max_num_test() {
        assert_eq!(Atom::Number(2.5f64),
                   max_num(get_input_num_and_sym()).unwrap());
    }

    #[test]
    fn min_num_test() {
        assert_eq!(Atom::Number(2f64),
                   min_num(get_input_num_and_sym()).unwrap());
    }

}
