//https://www.dcode.fr/reverse-polish-notation

mod algorithm;
mod dispositions;
mod formula;
mod postfix_to_infix;

use crate::algorithm::compute;

use clap::Parser;

use indicatif::{HumanDuration, ProgressBar, ProgressStyle};
use colored::Colorize;
use std::time::Instant;

#[derive(Debug, Parser)]
#[command(
    name = "newyearseve",
    about = "looking for a New Year's Eve count down formula"
)]

/// Command line arguments
struct Opt {
    /// number of parallel jobs (0 will use available cores)
    #[arg(short, long, default_value = "0")]
    jobs: usize,

    /// target year
    #[arg(short, long)]
    target: i64,

    /// output detailed results
    #[arg(short, long)]
    report: bool,

    /// use only simple operations (+, -, *)
    #[arg(short = 's', long)]
    simple_ops: bool,
}

fn main() {
    // parse command-line parameters
    let opt = Opt::parse();

    // set cores to be used
    let jobs = if opt.jobs == 0 {
        rayon::current_num_threads()
    } else {
        rayon::ThreadPoolBuilder::new()
            .num_threads(opt.jobs)
            .build_global()
            .unwrap();
        opt.jobs
    };

    // display applicationn header
    println!();
    println!("{}", "New Year's Eve countdown formula".green().bold());
    println!();
    println!("target : {}", opt.target.to_string().yellow());
    println!(
        "threads: {} (Rayon)",
        jobs.to_string().yellow()
    );
    println!("report : {}", opt.report.to_string().yellow());
    println!("simple ops: {}", opt.simple_ops.to_string().yellow());
    println!();

    // setup progress bar
    let pb = ProgressBar::new(0); // Sarà impostato nel compute
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {percent:>3}% ({eta})")
            .unwrap()
            .progress_chars("#>-")
    );

    // start timer
    let started = Instant::now();

    // compute results using Rayon for parallel processing
    let results = compute(opt.target, &pb, opt.simple_ops);

    // stop progress bar
    pb.finish_with_message("Computation complete");

    // stop duration timer and get total results
    let duration = started.elapsed();

    // sort results and remove duplicated if needed
    let mut results: Vec<String> = results.iter().map(|x| x.to_infix()).collect();

    // remove duplicate solutions
    results.sort();
    results.dedup();

    // sort by solution length (shorter first)
    results.sort_by_key(|x| x.len());

    // solutions found
    let solutions = results.len();

    // filter results
    if !opt.report {
        if let Some(min_len) = results.iter().map(|x| x.len()).min() {
            results = results.into_iter().filter(|x| x.len() == min_len).collect();
        }
    }

    // display detailed results
    println!();
    println!("{}", "Results:".green().bold());
    for (i, result) in results.iter().enumerate() {
        println!("{} : {}", i + 1, result);
    }

    // display summary results (i.e. duration and speed)
    println!();
    println!(
        "Found {} solutions in {}",
        solutions.to_string().yellow().bold(),
        HumanDuration(duration).to_string().green()
    );
}
