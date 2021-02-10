use crate::dispositions::DispositionsTrait;
use crate::formula::{Formula, FORMULA_NUM_OPERATORS, FORMULA_OPERATORS, FORMULA_SIZE};

use indicatif::ProgressBar;
use itertools::Itertools;

// computed out of the box
pub const MAX_ITERATIONS: u64 = 558_593_750;

pub fn compute(target: i64, part: usize, parts: usize, progress_bar: &ProgressBar) -> Vec<Formula> {
    // generate all valid operator's positions
    let pos = (0..FORMULA_SIZE)
        .combinations(FORMULA_NUM_OPERATORS as usize)
        .filter(|x| Formula::is_valid(x));

    // generate all possible operators dispositions
    let ops = FORMULA_OPERATORS.iter().cloned().dispositions_part(
        FORMULA_NUM_OPERATORS as usize,
        part as u8,
        parts as u8,
    );

    // looking for formula that compute a specific value
    let result: Vec<Formula> = pos
        .cartesian_product(ops)
        .inspect(|_| progress_bar.inc(1))
        .map(|(p, o)| Formula::new(p, o))
        .filter(|f| f.evaluate() == Some(target))
        .collect();

    // dispose progress bar
    progress_bar.finish_with_message("done");

    // return result
    result
}
