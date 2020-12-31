//https://www.dcode.fr/reverse-polish-notation

mod dispositions;

use crate::dispositions::*;
use itertools::Itertools;
use std::char;
use std::time::Instant;

const FORMULA_NUM_OPERANDS: u8 = 9;
const FORMULA_NUM_OPERATORS: u8 = 8;
const FORMULA_SIZE: u8 = FORMULA_NUM_OPERANDS + FORMULA_NUM_OPERATORS;

const FORMULA_OPERATORS: [char; 5] = ['+', '-', '*', '/', '^'];

#[derive(Debug)]
pub struct Formula<'a> {
    positions: Vec<u8>,
    operators: Vec<&'a char>,
}

fn generate_formula(f: &Formula) -> String {
    let &Formula {
        ref positions,
        ref operators,
    } = f;

    let mut o = 0;
    let mut d = 9;

    let mut buf = String::with_capacity(FORMULA_SIZE as usize);

    for i in 0..FORMULA_SIZE {
        if i < positions[o] {
            buf.push(char::from_digit(d, 10).unwrap());
            d -= 1;
        } else {
            buf.push(*operators[o]);
            o += 1;
        }
    }

    buf
}

fn evaluate_raw_formula(f: &Formula) -> Option<u32> {
    let &Formula {
        ref positions,
        ref operators,
    } = f;

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

fn main() {
    // formula string is 17 chars long (9 digits and 8 operators)
    // considering only binary operators then operator position
    // need to be p[i] >= 2*(i+1) otherwise there will not enough
    // operands

    let start = Instant::now();
    let mut count = 0;

    // generate all valid operator's positions
    let pos = (0..FORMULA_SIZE)
        .combinations(FORMULA_NUM_OPERATORS as usize)
        .filter(|x| x.iter().enumerate().all(|(i, v)| *v >= 2 * (i + 1) as u8));

    // generate all possible operators dispositions
    let ops = FORMULA_OPERATORS
        .iter()
        .dispositions(FORMULA_NUM_OPERATORS as usize);

    for (p, o) in pos.cartesian_product(ops) {
        let f = Formula {
            positions: p,
            operators: o,
        };

        let r = evaluate_raw_formula(&f).unwrap_or_default();

        if r == 2021 {
            println!("{:?} -> {:?}", generate_formula(&f), r);
        }

        count += 1;
    }

    println!(
        "{} iterations in {:?} @{}itms",
        count,
        start.elapsed(),
        count / start.elapsed().as_millis()
    );
}
