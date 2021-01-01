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

    pub fn evaluate(&self) -> Option<u32> {
        let Formula {
            ref positions,
            ref operators,
        } = self;

        let mut stack: Vec<u32> = Vec::with_capacity(FORMULA_NUM_OPERANDS as usize);

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
                    '+' => Some(op1 + op2),
                    '-' => Some(op1 - op2),
                    '*' => Some(op1 * op2),
                    '/' => {
                        if op2 != 0 && op1 % op2 == 0 {
                            Some(op1 / op2)
                        } else {
                            None
                        }
                    }
                    '^' => Some(op1.pow(op2)),
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
