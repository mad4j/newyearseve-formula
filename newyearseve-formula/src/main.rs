//https://www.dcode.fr/reverse-polish-notation

mod algorithm;
mod dispositions;
mod formula;
mod postfix_to_infix;

use crate::algorithm::{compute, MAX_ITERATIONS};
use crate::formula::Formula;

use structopt::StructOpt;

use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressDrawTarget, ProgressStyle};

use colored::Colorize;
use std::thread::{self, available_parallelism};
use std::time::Instant;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "newyearseve",
    about = "looking for a New Year's Eve count down formula"
)]

/// Command line arguments
struct Opt {
    /// number of parallel jobs (0 will uses available cores)
    #[structopt(short, long, default_value = "0")]
    jobs: usize,

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

    // retrives available cores
    let available_cores = match available_parallelism() {
        Ok(v) => v.into(),
        Err(_) => 1,
    };

    // set cores to be used
    let jobs = if opt.jobs == 0 {
        available_cores
    } else {
        opt.jobs
    };

    // display applicationn header
    println!();
    println!("{}", "New Year's Eve countdown formula".green().bold());
    println!();
    println!("target : {}", opt.target.to_string().yellow());
    println!(
        "jobs   : {} on {} cores",
        jobs.to_string().yellow(),
        available_cores.to_string().yellow()
    );
    println!("report : {}", opt.report.to_string().yellow());
    println!();

    // progress bar container setup
    let multi_bar = MultiProgress::new();
    let bar_style = ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {percent:>3}% ({eta}) {msg}")
        .unwrap()
        .progress_chars("#--");

    // avoid console bottleneck
    ProgressDrawTarget::stderr_with_hz(1);

    // start timer
    let started = Instant::now();

    let mut results = Vec::<Formula>::new();
    let mut handles = vec![];

    let iterations = MAX_ITERATIONS / jobs as u64;
    let target = opt.target;
    let cores = jobs;

    // initialize and start computation cores
    for i in 0..cores {
        //build a new progress bar
        let pb = multi_bar.add(ProgressBar::new(iterations));
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
    //multi_bar.join().unwrap()

    // collect results
    for h in handles {
        let mut r = h.join().unwrap();
        results.append(&mut r);
    }

    // stop duration timer and get total results
    let duration = started.elapsed();

    // sort results and remove duplicated if needed
    let mut results: Vec<String> = results.iter().map(|x| x.to_infix()).collect();
    //let mut results: Vec<String> = results.iter().map(|x| format!("'{}' - '{}' = {:?}", x.to_string(), x.to_infix(), eval_int(&x.to_infix()))).collect();

    // remove duplicate solutions
    results.sort();
    results.dedup();

    // sort by solution length (shorter first)
    results.sort_by_key(|x| x.len());

    // solutions found
    let solutions = results.len();

    // filter results
    if !opt.report {
        let min = results.iter().map(|x| x.len()).min().unwrap_or_default();
        results = results.into_iter().filter(|x| x.len() == min).collect();
    }

    // display detailed results
    println!();
    println!("{}", "Results:".green().bold());
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
