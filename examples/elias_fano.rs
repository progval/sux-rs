use clap::Parser;
use rand::rngs::SmallRng;
use rand::Rng;
use rand::SeedableRng;
use std::hint::black_box;
use sux::prelude::*;

use std::time::Instant;

#[derive(Parser, Debug)]
#[command(about = "Benchmarks the Rust Sux implementation", long_about = None)]
struct Args {
    /// The number of elements
    n: usize,

    /// The size of the universe
    u: usize,

    /// The number of values to test
    t: usize,

    /// The number of test repetitions
    #[arg(short, long, default_value = "0.5")]
    density: f64,

    /// The number of test repetitions
    #[arg(short, long, default_value = "10")]
    repeats: usize,
}

fn main() {
    let args = Args::parse();
    let mut values = Vec::with_capacity(args.n);
    let mut rng = SmallRng::seed_from_u64(0);
    for _ in 0..args.n {
        values.push(rng.gen_range(0..args.u));
    }
    values.sort();
    let mut elias_fano_builder = EliasFanoBuilder::new(args.u, args.n);
    for value in values {
        elias_fano_builder.push(value).unwrap();
    }
    let elias_fano: EliasFano<QuantumIndex<CountBitVec, Vec<usize>, 8>, CompactArray> =
        elias_fano_builder.build().convert_to().unwrap();

    let mut ranks = Vec::with_capacity(args.t);
    for _ in 0..args.t {
        ranks.push(rng.gen_range(0..args.n));
    }

    let mut u: usize = 0;

    for _ in 0..args.repeats {
        let start = Instant::now();
        for &rank in &ranks {
            unsafe {
                u += elias_fano.get_unchecked(rank);
            }
        }
        let duration = start.elapsed();

        println!(
            "EliasFano select {}ns",
            duration.as_secs_f64() * 1.0e9 / args.t as f64
        );
    }

    black_box(u);
}
