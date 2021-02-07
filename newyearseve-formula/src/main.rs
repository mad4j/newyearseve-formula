//https://www.dcode.fr/reverse-polish-notation

mod dispositions;
mod formula;

use crate::dispositions::*;
use crate::formula::*;

use indicatif::HumanDuration;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;

use itertools::Itertools;
use std::time::Instant;

const MAX_ITERATIONS: u64 = 558_593_750;
const MAX_PARTS: u8 = 4;

fn main() {

    // start timer
    let started = Instant::now();

    // progress bar setup
    let progress_bar = ProgressBar::new(MAX_ITERATIONS);
    progress_bar.set_draw_delta(10_000);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {percent:>3}% ({eta}) {msg}")
            .progress_chars("#--"),
    );

    // generate all valid operator's positions
    let pos = (0..FORMULA_SIZE)
        .combinations(FORMULA_NUM_OPERATORS as usize)
        .filter(|x| Formula::is_valid(x));

    // generate all possible operators dispositions
    let ops = FORMULA_OPERATORS
        .iter()
        .cloned()
        .dispositions(FORMULA_NUM_OPERATORS as usize);

    // looking for formula that compute a specific value
    let results: Vec<_> = pos
        .cartesian_product(ops)
        .inspect(|_| progress_bar.inc(1))
        .map(|(p, o)| Formula::new(p, o))
        .filter(|f| f.evaluate() == Some(2021))
        .collect();

    // dispose progress bar
    progress_bar.finish();

    // display results
    for r in &results {
        println!("{}", r);
    }

    // display duration
    println!(
        "found {} solutions in {} @{} it per millis",
        results.len(),
        HumanDuration(started.elapsed()),
        MAX_ITERATIONS / started.elapsed().as_millis() as u64
    );
}
