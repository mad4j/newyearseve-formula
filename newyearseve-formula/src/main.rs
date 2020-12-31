//https://www.dcode.fr/reverse-polish-notation

mod dispositions;
mod formula;

use crate::dispositions::*;
use crate::formula::*;
use itertools::Itertools;
use std::time::Instant;

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
        let f = Formula::new(p, o);
        let r = f.evaluate().unwrap_or_default();

        if r == 2021 {
            println!("{} -> {}", f, r);
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
