use crate::postfix_to_infix::to_infix;

use std::char;
use std::fmt;

pub const FORMULA_NUM_OPERANDS: u8 = 9;
pub const FORMULA_NUM_OPERATORS: u8 = 8;
pub const FORMULA_SIZE: u8 = FORMULA_NUM_OPERANDS + FORMULA_NUM_OPERATORS;

pub const FORMULA_OPERATORS: [char; 5] = ['+', '-', '*', '/', '^'];

#[derive(Debug)]
pub struct Formula {
    pub positions: Vec<u8>,
    pub operators: Vec<char>,
}

impl Formula {
    pub fn new(p: Vec<u8>, o: Vec<char>) -> Formula {
        Formula {
            positions: p,
            operators: o,
        }
    }

    // formula string is 17 chars long (9 digits and 8 operators)
    // considering only binary operators then operator position
    // need to be p[i] >= 2*(i+1) otherwise there will not enough
    // operands
    pub fn is_valid(v: &Vec<u8>) -> bool {
        v.iter().enumerate().all(|(i, v)| *v >= 2 * (i + 1) as u8)
    }

    pub fn evaluate(&self) -> Option<i64> {
        let Formula {
            ref positions,
            ref operators,
        } = self;

        let mut stack: Vec<i64> = Vec::with_capacity(FORMULA_NUM_OPERANDS as usize);

        let mut o = 0;
        let mut d = 9;

        for i in 0..FORMULA_SIZE {
            if i < positions[o] {
                stack.push(d);
                d -= 1;
            } else {
                let op2 = stack.pop()?;
                let op1 = stack.pop()?;
                let result = match operators[o] {
                    '+' => op1.checked_add(op2),
                    '-' => op1.checked_sub(op2),
                    '*' => op1.checked_mul(op2),
                    // works only if (op1%op2 == 0), otherwise discard
                    '/' => match op1.checked_rem(op2) {
                        Some(v) => {
                            if v == 0 {
                                op1.checked_div(op2)
                            } else {
                                None
                            }
                        }
                        // occurs when op2 == 0 (i.e. pushed result from previous computattions)
                        None => None,
                    },
                    '^' => op1.checked_pow(op2 as u32),
                    _ => None,
                };

                match result {
                    Some(v) => stack.push(v),
                    None => return None,
                }
                o += 1;
            }
        }

        if stack.len() == 1 {
            stack.pop()
        } else {
            None
        }
    }

    pub fn to_infix(&self) -> String {
        let s = format!("{}", self);
        to_infix(&s).unwrap()
    }
}

impl fmt::Display for Formula {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Formula {
            ref positions,
            ref operators,
        } = self;

        let mut o = 0;
        let mut d = 9;

        let mut buf = String::with_capacity(FORMULA_SIZE as usize);

        for i in 0..FORMULA_SIZE {
            if i < positions[o] {
                buf.push(char::from_digit(d, 10).unwrap());
                d -= 1;
            } else {
                buf.push(operators[o]);
                o += 1;
            }
        }

        write!(f, "{}", buf)
    }
}

#[cfg(test)]
mod tests {
    use crate::formula::Formula;

    #[test]
    fn it_works() {
        let f = Formula::new(
            vec![2, 4, 6, 8, 10, 12, 14, 16],
            vec!['+', '+', '+', '+', '+', '+', '+', '+'],
        );
        println!("{}", f);
        println!("{:?}", f.evaluate());
        assert!(true);
    }
}
