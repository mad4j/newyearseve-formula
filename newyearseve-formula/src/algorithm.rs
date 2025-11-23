use crate::dispositions::DispositionsTrait;
use crate::formula::{Formula, FORMULA_NUM_OPERATORS, FORMULA_OPERATORS, FORMULA_SIZE};

use indicatif::ProgressBar;
use itertools::Itertools;
use rayon::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};

pub fn compute(target: i64, progress_bar: &ProgressBar, simple_ops: bool) -> Vec<Formula> {
    // generate all valid operator's positions
    let positions: Vec<Vec<u8>> = (0..FORMULA_SIZE)
        .combinations(FORMULA_NUM_OPERATORS as usize)
        .filter(|x| Formula::is_valid(x))
        .collect();

    // select operators based on simple_ops flag
    let available_operators: Vec<char> = if simple_ops {
        vec!['+', '-', '*']
    } else {
        FORMULA_OPERATORS.to_vec()
    };

    // generate all possible operators dispositions
    let operators: Vec<Vec<char>> = available_operators
        .iter()
        .cloned()
        .dispositions(FORMULA_NUM_OPERATORS as usize)
        .collect();

    let total_combinations = positions.len() * operators.len();
    progress_bar.set_length(total_combinations as u64);

    // counter for progress tracking
    let counter = AtomicU64::new(0);
    
    // looking for formula that compute a specific value using Rayon
    let result: Vec<Formula> = positions
        .into_par_iter()
        .map(|pos| {
            operators
                .par_iter()
                .filter_map(|ops| {
                    let formula = Formula::new(pos.clone(), ops.clone());
                    let current = counter.fetch_add(1, Ordering::Relaxed);
                    
                    // Update progress every 10000 iterations to avoid overhead
                    if current % 10000 == 0 {
                        progress_bar.set_position(current);
                    }
                    
                    if formula.evaluate() == Some(target) {
                        Some(formula)
                    } else {
                        None
                    }
                })
                .collect::<Vec<Formula>>()
        })
        .flatten()
        .collect();

    // finish progress bar
    progress_bar.set_position(total_combinations as u64);
    progress_bar.finish_with_message("Complete");

    result
}
