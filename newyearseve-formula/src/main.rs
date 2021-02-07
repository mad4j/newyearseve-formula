//https://www.dcode.fr/reverse-polish-notation

mod dispositions;
mod formula;

use indicatif::HumanDuration;
use crate::dispositions::*;
use crate::formula::*;
use indicatif::ProgressStyle;

use itertools::Itertools;
use std::time::Instant;

use indicatif::ProgressBar;

const MAX_ITERATIONS: u64 = 558_593_750;

fn main() {
    // formula string is 17 chars long (9 digits and 8 operators)
    // considering only binary operators then operator position
    // need to be p[i] >= 2*(i+1) otherwise there will not enough
    // operands

    let start = Instant::now();

    let bar = ProgressBar::new(MAX_ITERATIONS);
    bar.set_draw_delta(10_000);
    bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {percent:>3}% ({eta}) {msg}")
            .progress_chars("##-"),
    );

    // generate all valid operator's positions
    let pos = (0..FORMULA_SIZE)
        .combinations(FORMULA_NUM_OPERATORS as usize)
        .filter(|x| x.iter().enumerate().all(|(i, v)| *v >= 2 * (i + 1) as u8));

    // generate all possible operators dispositions
    let ops = FORMULA_OPERATORS
        .iter()
        .cloned()
        .dispositions(FORMULA_NUM_OPERATORS as usize);

    // looking for formula that compute a specific value
    let results: Vec<_> = pos
        .cartesian_product(ops)
        .inspect(|_| bar.inc(1))
        .map(|(p, o)| Formula::new(p, o))
        .filter_map(|f| f.evaluate())
        .filter(|x| *x == 2021)
        .collect();

    bar.finish();

    // display results
    for r in results {
        println!("{}", r);
    }

    // display duration
    println!(
        "{} iterations in {:?} @{}ipms",
        MAX_ITERATIONS,
        //start.elapsed(),
        HumanDuration(start.elapsed()),
        MAX_ITERATIONS / start.elapsed().as_millis() as u64
    );
}
