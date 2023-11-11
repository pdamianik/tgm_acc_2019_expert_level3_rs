const N: u128 = 10;

use clap::{Parser, Subcommand, ValueEnum};
use tgm_acc_2019_expert_level3::{cycle, hardcoded, initial, linear, parallel, rayon};

#[derive(Parser)]
#[command(
    author,
    version,
    about,
    long_about = "A collection of solutions for the problem S[0]=290797 S[n+1]=S[n]^2 mod 50515093 Let A(i,j) min S[i],S[i+1],…,S[j] for i≤j. Let M(N) = ∑A(i,j) for 1≤i≤j≤N. M(n)=?"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Solve the problem
    Solve {
        /// The algorithm to solve the problem with
        #[arg(value_enum)]
        algorithm: Algorithms,
        /// The argument n for the function m
        #[arg(default_value_t = N)]
        n: u128,
    },
}

#[derive(Clone, Copy, PartialEq, Eq, ValueEnum)]
enum Algorithms {
    /// The algorithm as the exercise puts it (WARNING doesn't scale well)
    /// time: O(n^4), mem: O(1))
    Initial,
    /// The same as initial
    I,
    /// A parallel version utilising rayon with a slight complexity optimization
    /// (time: O(n^2), mem: O(n))
    Rayon,
    /// The same as rayon
    R,
    /// The rayon version implemented with rust concurrency types and os threads
    /// (time: O(n^2), mem: O(n))
    Parallel,
    /// The same as parallel
    P,
    /// A solution that takes (roughly) linear time and little memory
    /// (time: O(~n), mem: O(~1))
    Linear,
    /// The same as linear
    L,
    /// A solution that uses the fact that the generator loops with a global minimum of 3
    /// (time: O(~1), mem: O(~1))
    Cycle,
    /// The same as cycle
    C,
    /// The cycle solution with hardcoded values
    /// (time: O(~1), mem: O(~1))
    Hardcoded,
    /// The same as hardcoded
    H,
}

fn main() {
    let Cli { command } = Cli::parse();

    match command {
        Commands::Solve { algorithm, n } => {
            let result = match algorithm {
                Algorithms::Initial | Algorithms::I => initial::m(n as usize),
                Algorithms::Rayon | Algorithms::R => rayon::m(n as usize),
                Algorithms::Parallel | Algorithms::P => parallel::m(n as usize),
                Algorithms::Linear | Algorithms::L => linear::m(n as usize),
                Algorithms::Cycle | Algorithms::C => cycle::m(n),
                Algorithms::Hardcoded | Algorithms::H => hardcoded::m(n),
            };

            println!("M({}) = {}", n, result);
        }
    };
}
