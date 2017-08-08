extern crate stopwatch;

mod m;

use stopwatch::Stopwatch;
use std::env;
use std::fs::{self, File};
use std::process;

fn run() -> m::Result<u64> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: program <# of renames>");
        process::exit(1);
    }

    let count: u64 = args[1].parse()?;
    const FIRST_FILE_NAME: &str = "0.txt";

    // write and close immediately
    File::create(FIRST_FILE_NAME)?;

    let mut sw = Stopwatch::start_new();

    // rename in a loop
    for index in 0..count {
        let curr_file_name = format!("{}.txt", index);
        let next_file_name = format!("{}.txt", index + 1);
        fs::rename(&curr_file_name, &next_file_name)?
    }

    sw.stop();
    println!("Duration: {} ms", sw.elapsed_ms());

    Ok(count)
}

fn main() {
    match run() {
        Ok(count) => println!("Program completed with {} renames!", count),
        Err(e) => println!("Error: {}", e),
    }
}
