//https://www.dcode.fr/reverse-polish-notation

use itertools::Itertools;
use std::char;

const FORMULA_OPERANDS: usize = 9;
const FORMULA_OPERATORS: usize = 8;
const FORMULA_SIZE: usize = FORMULA_OPERANDS + FORMULA_OPERATORS;

#[derive(Debug)]
pub struct UnpackIterator {
    value: usize,
    modulo: usize,
    length: usize,
    index: usize,
}

impl UnpackIterator {
    pub fn init(value: usize, modulo: usize, length: usize) -> UnpackIterator {
        UnpackIterator {
            value,
            modulo,
            length,
            index: 0,
        }
    }
}

impl Iterator for UnpackIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let &mut UnpackIterator {
            ref mut value,
            ref modulo,
            ref mut length,
            ref mut index,
        } = self;

        if index < length {
            let v = *value % modulo;

            *index += 1;
            *value /= modulo;

            Some(v)
        } else {
            None
        }
    }
}


#[derive(Debug, Clone)]
pub struct Dispositions<I: Iterator> {
    elems: Vec<I::Item>,
    length: usize,
    max_index: usize,
    index: usize,
}

impl<I> Iterator for Dispositions<I>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.max_index {
            let &mut Dispositions {
                ref elems,
                ref length,
                max_index: _,
                ref mut index,
            } = self;

            *index += 1;

            Some(
                UnpackIterator::init(*index, elems.len(), *length)
                    .map(|x| self.elems[x].clone())
                    .collect(),
            )
        } else {
            None
        }
    }
}


pub fn dispositions<I: Iterator>(iter: I, k: usize) -> Dispositions<I> {
    let elems: Vec<I::Item> = iter.collect();
    let max_index: usize = if k == 0 { 0 } else { elems.len().pow(k as u32) };

    Dispositions {
        elems,
        length: k,
        max_index,
        index: 0,
    }
}

pub trait DispositionsTrait: Sized + Iterator {
    fn dispositions(self, k: usize) -> Dispositions<Self>;
}

impl<I: Iterator> DispositionsTrait for I {
    fn dispositions(self, k: usize) -> Dispositions<Self> {
        dispositions(self, k)
    }
}

/*
fn permutations(self, k: usize) -> Permutations<Self>
        where Self: Sized,
              Self::Item: Clone
    {
        permutations::permutations(self, k)
    }
*/


fn generate_formula(positions: &[usize], operations: &[&char]) -> String {
    let mut o = 0;
    let mut d = 9;

    let mut buf = String::with_capacity(2 * FORMULA_SIZE);

    for i in 0..17 {
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
    let mut stack: Vec<f32> = Vec::new();

    for token in expr.chars() {
        let result: Option<f32> = if token.is_digit(10) {
            match token.to_digit(10) {
                Some(v) => Some(v as f32),
                None => None,
            }
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

    // generate all valid operator's positions
    let pos = (0..FORMULA_SIZE)
        .combinations(FORMULA_OPERATORS)
        .filter(|x| x.iter().enumerate().all(|(i, v)| *v >= 2 * (i + 1)));

    // generate all possible operators dispositions
    let ops = ['+', '-', '*', '/', '^']
        .iter()
        .dispositions(FORMULA_OPERATORS);

    
    for (p, o) in pos.cartesian_product(ops) {

        let s = generate_formula(&p[..], &o[..]);
        let r = evaluate_formula(&s).unwrap_or_default();

        if r == 2021f32 {
            println!("{:?} -> {:?}", s, r);
        }
    }

    // "98+7+6+5+432--1--" -> -29.0

    //println!("{:?}", evaluate_formula("98+"));
    //println!("{:?}", evaluate_formula("98+7+"));
    //println!("{:?}", evaluate_formula("98+7+6+"));
    //println!("{:?}", evaluate_formula("98+7+6+5+"));
    //println!("{:?}", evaluate_formula("98+7+6+5+432--1--"));

    //println!("{:?}", evaluate_formula("98*765^4/++3-2+1-"));
}
