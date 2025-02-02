use clap::Parser;
use dsi_progress_logger::ProgressLogger;
use rand::rngs::SmallRng;
use rand::Rng;
use rand::SeedableRng;
use std::hint::black_box;
use sux::prelude::CompactArray;
use sux::prelude::*;

#[derive(Parser, Debug)]
#[command(about = "Benchmarks compact arrays", long_about = None)]
struct Args {
    /// The width of the elements of the array
    width: usize,
    /// The base-2 logarith of the length of the array
    log2_size: usize,

    /// The number of test repetitions
    #[arg(short, long, default_value = "10")]
    repeats: usize,

    /// The number of elements to get and set
    #[arg(short, long, default_value = "10000000")]
    n: usize,
}

pub fn main() {
    stderrlog::new()
        .verbosity(2)
        .timestamp(stderrlog::Timestamp::Second)
        .init()
        .unwrap();

    let args = Args::parse();

    let mut a = CompactArray::new(args.width, 1 << args.log2_size);
    let mask = (1 << args.log2_size) - 1;

    let mut pl = ProgressLogger::default();
    let mut u = 0;

    for _ in 0..args.repeats {
        let mut rand = SmallRng::seed_from_u64(0);
        pl.item_name = "write";
        pl.start("Writing...");
        for _ in 0..args.n {
            let x = rand.gen::<usize>() & mask;
            unsafe { a.set_unchecked(x, 1) };
        }
        pl.done_with_count(args.n);

        pl.item_name = "read";
        pl.start("Reading...");
        for _ in 0..args.n {
            unsafe {
                u += a.get_unchecked(rand.gen::<usize>() & mask);
            }
        }
        pl.done_with_count(args.n);
    }

    black_box(u);
}
