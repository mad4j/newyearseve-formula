#[derive(Debug, Clone)]
pub struct Exp {
    value: String,
    prec: u8,
}

impl Exp {
    fn new<S>(value: S, prec: u8) -> Exp
    where
        S: Into<String>,
    {
        Exp {
            value: value.into(),
            prec,
        }
    }
}

fn op_prec(c: char) -> u8 {
    match c {
        '+' | '-' => 1,
        '/' | '*' => 2,
        '^' => 3,
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => 10,
        _ => 0,
    }
}

fn op_right_assoc(c: char) -> bool {
    match c {
        '^' => true,
        '+' | '-' | '*' | '/' => false,
        _ => false,
    }
}

fn is_operator(c: char) -> bool {
    match c {
        '+' | '-' | '/' | '*' | '^' => true,
        _ => false,
    }
}

pub fn to_infix<S>(formula: S) -> Option<String>
where
    S: Into<String>,
{
    let mut stack: Vec<Exp> = Vec::new();

    for c in formula.into().chars() {
        if is_operator(c) {
            let r = stack.pop()?;
            let l = stack.pop()?;

            let p = op_prec(c);
            let a = op_right_assoc(c);

            let tr = if r.prec < p || (!a && r.prec == p) {
                format!("({})", r.value)
            } else {
                r.value
            };

            let tl = if l.prec < p || (a && l.prec == p) {
                format!("({})", l.value)
            } else {
                l.value
            };

            stack.push(Exp::new(format!("{}{}{}", tl, c, tr), p));
        } else {
            stack.push(Exp::new(c.to_string(), op_prec(c)));
        }
    }

    if stack.len() == 1 {
        Some(stack.pop()?.value)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn postfix_to_infix_test_01() {
        assert_eq!(
            to_infix("12+34+56+^^"),
            Some(String::from("(1+2)^(3+4)^(5+6)"))
        );
        assert_eq!(
            to_infix("342*15-23^^/+"),
            Some(String::from("3+4*2/(1-5)^2^3"))
        );
        assert_eq!(
            to_infix("12+34+^56+^"),
            Some(String::from("((1+2)^(3+4))^(5+6)"))
        );
    }
}
