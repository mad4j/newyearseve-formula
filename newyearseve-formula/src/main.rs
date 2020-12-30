//https://www.dcode.fr/reverse-polish-notation

mod dispositions;

use crate::dispositions::*;
use itertools::Itertools;
use std::char;
use std::time::{Duration, Instant};

const FORMULA_NUM_OPERANDS: usize = 9;
const FORMULA_NUM_OPERATORS: usize = 8;
const FORMULA_SIZE: usize = FORMULA_NUM_OPERANDS + FORMULA_NUM_OPERATORS;

const FORMULA_OPERATORS: [char; 5] = ['+', '-', '*', '/', '^'];

fn generate_formula(positions: &[usize], operations: &[&char]) -> String {
    let mut o = 0;
    let mut d = 9;

    let mut buf = String::with_capacity(FORMULA_SIZE);

    for i in 0..FORMULA_SIZE {
        if i < positions[o] {
            buf.push(char::from_digit(d, 10).unwrap());
            d -= 1;
        } else {
            buf.push(*operations[o]);
            o += 1;
        }
    }

    buf
}

fn evaluate_formula(expr: &str) -> Option<f32> {
    let mut stack: Vec<f32> = Vec::with_capacity(FORMULA_NUM_OPERANDS);

    for token in expr.chars() {
        let result = if token.is_digit(10) {
            // convert char to digit and then to float
            token.to_digit(10).and_then(|x| Some(x as f32))
        } else {
            let op2 = stack.pop()?;
            let op1 = stack.pop()?;

            match token {
                '+' => Some(op1 + op2),
                '-' => Some(op1 - op2),
                '*' => Some(op1 * op2),
                '/' => Some(op1 / op2),
                '^' => Some(op1.powf(op2)),
                _ => None,
            }
        };

        match result {
            Some(v) => stack.push(v),
            None => (),
        };
    }

    stack.pop()
}

fn main() {
    // formula string is 17 chars long (9 digits and 8 operators)
    // considering only binary operators then operator position
    // need to be p[i] >= 2*(i+1) otherwise there will not enough
    // operands

    let start = Instant::now();

    // generate all valid operator's positions
    let pos = (0..FORMULA_SIZE)
        .combinations(FORMULA_NUM_OPERATORS)
        .filter(|x| x.iter().enumerate().all(|(i, v)| *v >= 2 * (i + 1)));

    // generate all possible operators dispositions
    let ops = FORMULA_OPERATORS.iter().dispositions(FORMULA_NUM_OPERATORS);

    for (p, o) in pos.cartesian_product(ops) {
        let s = generate_formula(&p[..], &o[..]);
        let r = evaluate_formula(&s).unwrap_or_default();

        if r == 2021f32 {
            println!("{:?} -> {:?}", s, r);
        }
    }

    println!("Time elapsed is {:?}", start.elapsed());
}
