use crate::postfix_to_infix::to_infix;

use std::char;
use std::fmt;

pub const FORMULA_NUM_OPERANDS: u8 = 9;
pub const FORMULA_NUM_OPERATORS: u8 = 8;
pub const FORMULA_SIZE: u8 = FORMULA_NUM_OPERANDS + FORMULA_NUM_OPERATORS;

pub const FORMULA_OPERATORS: [char; 6] = ['+', '-', '*', '/', '^', '&'];

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

        // Stack of (value, is_original_digit)
        let mut stack: Vec<(i64, bool)> = Vec::with_capacity(FORMULA_NUM_OPERANDS as usize);

        let mut o = 0;
        let mut d = 9;

        for i in 0..FORMULA_SIZE {
            if i < positions[o] {
                stack.push((d, true)); // true = original digit
                d -= 1;
            } else {
                let (op2_val, op2_is_digit) = stack.pop()?;
                let (op1_val, op1_is_digit) = stack.pop()?;
                let result = match operators[o] {
                    '+' => op1_val.checked_add(op2_val),
                    '-' => op1_val.checked_sub(op2_val),
                    '*' => op1_val.checked_mul(op2_val),
                    // works only if (op1_val%op2_val == 0), otherwise discard
                    '/' => match op1_val.checked_rem(op2_val) {
                        Some(v) => {
                            if v == 0 {
                                op1_val.checked_div(op2_val)
                            } else {
                                None
                            }
                        }
                        // occurs when op2_val == 0 (i.e. pushed result from previous computattions)
                        None => None,
                    },
                    '^' => op1_val.checked_pow(op2_val as u32),
                    // concatenation: 9 & 8 = 98, only between original digits
                    '&' => {
                        if op1_is_digit && op2_is_digit && op1_val >= 0 && op2_val >= 0 {
                            let concatenated = format!("{}{}", op1_val, op2_val);
                            concatenated.parse::<i64>().ok()
                        } else {
                            None
                        }
                    },
                    _ => None,
                };

                match result {
                    Some(v) => stack.push((v, false)), // false = computed result
                    None => return None,
                }
                o += 1;
            }
        }

        if stack.len() == 1 {
            Some(stack.pop()?.0)
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
