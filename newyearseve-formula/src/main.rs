//https://www.dcode.fr/reverse-polish-notation

mod dispositions;
mod formula;

use crate::dispositions::*;
use crate::formula::*;

use itertools::Itertools;
use std::time::Instant;

//use rayon::prelude::*;

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

    // looking for formula that compute a specific value
    pos.cartesian_product(ops)
        .map(|(p, o)| {
            count += 1;
            Formula::new(p, o)
        })
        .filter(|f| f.evaluate().unwrap_or_default() == 2021)
        .for_each(|f| println!("{}", f));

    println!(
        "{} iterations in {:?} @{}ipms",
        count,
        start.elapsed(),
        count / start.elapsed().as_millis()
    );
}
