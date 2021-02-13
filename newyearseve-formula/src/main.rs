//https://www.dcode.fr/reverse-polish-notation

mod algorithm;
mod dispositions;
mod formula;
mod integer_pack;
mod postfix_to_infix;

use crate::algorithm::{compute, MAX_ITERATIONS};
use crate::formula::Formula;

use structopt::StructOpt;

use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressStyle};

use colored::Colorize;
use std::thread;
use std::time::Instant;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "newyearseve",
    about = "looking for a New Year's Eve count down formula"
)]
struct Opt {
    /// number of parallel cores
    #[structopt(short, long, default_value = "1")]
    cores: usize,

    /// target year
    #[structopt(short, long)]
    target: i64,

    /// output detailed results
    #[structopt(short, long)]
    report: bool,
}

fn main() {
    // parse command-line parameters
    let opt = Opt::from_args();

    // display applicationn header
    println!();
    println!("{}", "New Year's Eve countdown formula".green().bold());
    println!();
    println!("target : {}", opt.target.to_string().yellow());
    println!("cores  : {}", opt.cores.to_string().yellow());
    println!("report : {}", opt.report.to_string().yellow());
    println!();

    // progress bar container setup
    let multi_bar = MultiProgress::new();
    let bar_style = ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {percent:>3}% ({eta}) {msg}")
        .progress_chars("#--");

    // start timer
    let started = Instant::now();

    let mut results = Vec::<Formula>::new();
    let mut handles = vec![];

    let iterations = MAX_ITERATIONS / opt.cores as u64;
    let target = opt.target;
    let cores = opt.cores;

    // initialize and start computeation cores
    for i in 0..cores {
        //build a new progress bar
        let pb = multi_bar.add(ProgressBar::new(iterations));
        pb.set_draw_delta(iterations / 100);
        pb.set_style(bar_style.clone());

        // start requested cores
        let handle = thread::spawn(move || {
            // compute domain search
            let result = compute(target, i, cores, &pb);

            // return result
            result
        });

        // store thread handle to retrieve results later
        handles.push(handle);
    }

    // waits for all progress bars to report that they are finished.
    multi_bar.join().unwrap();

    // collect results
    for h in handles {
        let mut r = h.join().unwrap();
        results.append(&mut r);
    }

    // stop duration timer and get total results
    let duration = started.elapsed();
    let solutions = results.len();

    // sort results and remove duplicated if needed
    let mut results: Vec<String> = results.iter().map(|x| x.to_infix()).collect();
    results.sort_by_key(|x| x.len());
    results.dedup();

    // filter results
    if !opt.report {
        let min = results.iter().map(|x| x.len()).min().unwrap_or_default();
        results = results.into_iter().filter(|x| x.len() == min).collect();
    }

    // display detailed results
    println!();
    for i in 0..results.len() {
        println!("{} : {}", i + 1, results[i]);
    }

    // display summary results (i.e. duration and speed)
    println!();
    println!(
        "Found {} solutions in {} @{} iter per millis",
        solutions.to_string().yellow().bold(),
        HumanDuration(duration).to_string().green(),
        (MAX_ITERATIONS / duration.as_millis() as u64)
            .to_string()
            .green()
    );
}
