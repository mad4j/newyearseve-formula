//https://www.dcode.fr/reverse-polish-notation

mod dispositions;
mod formula;

use crate::dispositions::*;
use crate::formula::*;

use indicatif::HumanDuration;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;

use itertools::Itertools;

use std::thread;
use std::time::Instant;

const MAX_ITERATIONS: u64 = 558_593_750;
const MAX_PARTS: usize = 4;

fn compute(
    target: i64,
    part: usize,
    parts: usize, /*, progress_bar: &ProgressBar */
) -> Vec<Formula> {
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
        //.inspect(|_| progress_bar.inc(1))
        .map(|(p, o)| Formula::new(p, o))
        .filter(|f| f.evaluate() == Some(target))
        .collect();

    // return result
    result
}

fn main() {
    // progress bar setup
    let progress_bar = ProgressBar::new(MAX_ITERATIONS);
    progress_bar.set_draw_delta(10_000);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {percent:>3}% ({eta}) {msg}")
            .progress_chars("#--"),
    );

    // start timer
    let started = Instant::now();

    let mut results = Vec::<Formula>::new();
    let mut handles = vec![];

    for i in 0..MAX_PARTS {
        let handle = thread::spawn(move || compute(2021, i, MAX_PARTS));
        handles.push(handle);
    }

    for h in handles {
        let mut r = h.join().unwrap();
        results.append(&mut r);
    }

    // stop duration timer
    let duration = started.elapsed();

    // dispose progress bar
    progress_bar.finish();

    // display results
    for r in &results {
        println!("{}", r);
    }

    // display duration
    println!(
        "found {} solutions in {} @{} iter per millis",
        results.len(),
        HumanDuration(duration),
        MAX_ITERATIONS / duration.as_millis() as u64
    );
}
