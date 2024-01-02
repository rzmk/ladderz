use clap::Subcommand;

#[derive(Subcommand)]
#[command(arg_required_else_help(true))]
pub enum Prealgebra {
    /// Finds all factor pairs for a positive integer.
    /// 
    /// ## Example
    /// 
    /// ### Input
    /// 
    /// ```bash
    /// lz prealgebra factor-pairs 12
    /// ```
    /// 
    /// ### Output
    /// 
    /// ```bash
    /// The factor pairs of 12 are [(1, 12), (2, 6), (3, 4)].
    /// ```
    /// 
    /// ## Raw Output (use `-r` or `--raw`)
    /// 
    /// ```bash
    /// [(1, 12), (2, 6), (3, 4)]
    /// ```
    FactorPairs {
        /// The positive integer to find factor pairs for.
        n: u32,
        /// Whether or not to return the raw output.
        #[arg(short = 'r', long)]
        raw: bool,
    },
    /// Finds all factors for a positive integer.
    ///
    /// ## Example
    /// 
    /// ### Input
    /// 
    /// ```bash
    /// lz prealgebra factors 12
    /// ```
    /// 
    /// ### Output
    /// 
    /// ```bash
    /// The factors of 12 are [1, 2, 3, 4, 6, 12].
    /// ```
    /// 
    /// ## Raw Output (use `-r` or `--raw`)
    /// 
    /// ```bash
    /// [1, 2, 3, 4, 6, 12]
    /// ```
    Factors {
        /// The positive integer to find factors for.
        n: u32,
        /// Whether or not to return the raw output.
        #[arg(short = 'r', long)]
        raw: bool,
    },
    /// Finds all multiples of a positive integer in a given range.
    /// 
    /// ## Example
    /// 
    /// ### Input
    /// 
    /// ```bash
    /// lz prealgebra multiples-in-range 3 10
    /// ```
    /// 
    /// ### Output
    /// 
    /// ```bash
    /// The multiples of 3 in the range [1, 10] are [3, 6, 9].
    /// ```
    /// 
    /// ## Raw Output (use `-r` or `--raw`)
    /// 
    /// ```bash
    /// [3, 6, 9]
    /// ```
    MultiplesInRange {
        /// The positive integer to find multiples for.
        n: u32,
        /// The upper bound of the range to find multiples in.
        upper_bound: u32,
        /// Whether or not to return the raw output.
        #[arg(short = 'r', long)]
        raw: bool,
    },
    /// Finds all primes in a given range.
    /// 
    /// ## Example
    /// 
    /// ### Input
    /// 
    /// ```bash
    /// lz prealgebra primes-in-range 1 10
    /// ```
    /// 
    /// ### Output
    /// 
    /// ```bash
    /// The primes in the range [1, 10] are [2, 3, 5, 7].
    /// ```
    /// 
    /// ## Raw Output (use `-r` or `--raw`)
    /// 
    /// ```bash
    /// [2, 3, 5, 7]
    /// ```
    PrimesInRange {
        /// The lower bound of the range to find primes in.
        lower_bound: u32,
        /// The upper bound of the range to find primes in.
        upper_bound: u32,
        /// Whether or not to return the raw output.
        #[arg(short = 'r', long)]
        raw: bool,
    },
    /// Finds the prime factorization of a positive integer.
    /// 
    /// ## Example
    /// 
    /// ### Input
    /// 
    /// ```bash
    /// lz prealgebra prime-factorization 12
    /// ```
    /// 
    /// ### Output
    /// 
    /// ```bash
    /// The prime factorization of 12 is {2: 2, 3: 1}.
    /// ```
    /// 
    /// ## Raw Output (use `-r` or `--raw`)
    /// 
    /// ```bash
    /// {2: 2, 3: 1}
    /// ```
    PrimeFactorization {
        /// The positive integer to find the prime factorization of.
        n: u32,
        /// Whether or not to return the raw output.
        #[arg(short = 'r', long)]
        raw: bool,
    },
    /// Determines if a positive integer is composite.
    /// 
    /// ## Example
    /// 
    /// ### Input
    /// 
    /// ```bash
    /// lz prealgebra is-composite 12
    /// ```
    /// 
    /// ### Output
    /// 
    /// ```bash
    /// 12 is composite.
    /// ```
    /// 
    /// ## Raw Output (use `-r` or `--raw`)
    /// 
    /// ```bash
    /// true
    /// ```
    IsComposite {
        /// The positive integer to determine if it is composite.
        n: u32,
        /// Whether or not to return the raw output.
        #[arg(short = 'r', long)]
        raw: bool,
    },
    /// Determines if a positive integer is prime.
    /// 
    /// ## Example
    /// 
    /// ### Input
    /// 
    /// ```bash
    /// lz prealgebra is-prime 12
    /// ```
    /// 
    /// ### Output
    /// 
    /// ```bash
    /// 12 is not prime.
    /// ```
    /// 
    /// ## Raw Output (use `-r` or `--raw`)
    /// 
    /// ```bash
    /// false
    /// ```
    IsPrime {
        /// The positive integer to determine if it is prime.
        n: u32,
        /// Whether or not to return the raw output.
        #[arg(short = 'r', long)]
        raw: bool,
    },
    /// Determines if a positive integer is a factor of another positive integer.
    /// 
    /// ## Example
    /// 
    /// ### Input
    /// 
    /// ```bash
    /// lz prealgebra is-factor 3 12
    /// ```
    /// 
    /// ### Output
    /// 
    /// ```bash
    /// 3 is a factor of 12.
    /// ```
    /// 
    /// ## Raw Output (use `-r` or `--raw`)
    /// 
    /// ```bash
    /// true
    /// ```
    IsFactor {
        /// The positive integer to determine if it is a factor.
        n: u32,
        /// The positive integer to determine if it is a multiple.
        m: u32,
        /// Whether or not to return the raw output.
        #[arg(short = 'r', long)]
        raw: bool,
    },
    /// Determines if a positive integer is a multiple of another positive integer.
    /// 
    /// ## Example
    /// 
    /// ### Input
    /// 
    /// ```bash
    /// lz prealgebra is-multiple 12 3
    /// ```
    /// 
    /// ### Output
    /// 
    /// ```bash
    /// 12 is a multiple of 3.
    /// ```
    /// 
    /// ## Raw Output (use `-r` or `--raw`)
    /// 
    /// ```bash
    /// true
    /// ```
    IsMultiple {
        /// The positive integer to determine if it is a multiple.
        n: u32,
        /// The positive integer to determine if it is a factor.
        m: u32,
        /// Whether or not to return the raw output.
        #[arg(short = 'r', long)]
        raw: bool,
    },
}

pub fn match_prealgebra(function: Option<Prealgebra>) {
    use ladderz::prealgebra::*;
    match function {
        Some(Prealgebra::FactorPairs { n, raw }) => match raw {
            true => println!("{:?}", get_factor_pairs(n)),
            false => println!("The factor pairs of {} are {:?}.", n, get_factor_pairs(n)),
        },
        Some(Prealgebra::Factors { n, raw }) => match raw {
            true => println!("{:?}", get_factors(n)),
            false => println!("The factors of {} are {:?}.", n, get_factors(n)),
        },
        Some(Prealgebra::MultiplesInRange {
            n,
            upper_bound,
            raw,
        }) => match raw {
            true => println!("{:?}", get_multiples_in_range(n, upper_bound)),
            false => println!(
                "The multiples of {} in the range [1, {}] are {:?}.",
                n,
                upper_bound,
                get_multiples_in_range(n, upper_bound)
            ),
        },
        Some(Prealgebra::PrimesInRange {
            lower_bound,
            upper_bound,
            raw,
        }) => match raw {
            true => println!("{:?}", get_primes_in_range(lower_bound, upper_bound)),
            false => println!(
                "The primes in the range [{}, {}] are {:?}.",
                lower_bound,
                upper_bound,
                get_primes_in_range(lower_bound, upper_bound)
            ),
        },
        Some(Prealgebra::PrimeFactorization { n, raw }) => match raw {
            true => println!("{:?}", get_prime_factorization(n)),
            false => println!(
                "The prime factorization of {} is {:?}.",
                n,
                get_prime_factorization(n)
            ),
        },
        Some(Prealgebra::IsComposite { n, raw }) => match raw {
            true => println!("{:?}", is_composite(n)),
            false => println!(
                "{} is {}composite.",
                n,
                if is_composite(n) { "" } else { "not " }
            ),
        },
        Some(Prealgebra::IsPrime { n, raw }) => match raw {
            true => println!("{:?}", is_prime(n)),
            false => println!("{} is {}prime.", n, if is_prime(n) { "" } else { "not " }),
        },
        Some(Prealgebra::IsFactor { n, m, raw }) => match raw {
            true => println!("{:?}", is_factor(n, m)),
            false => println!(
                "{} is {}a factor of {}.",
                n,
                if is_factor(n, m) { "" } else { "not " },
                m
            ),
        },
        Some(Prealgebra::IsMultiple { n, m, raw }) => match raw {
            true => println!("{:?}", is_multiple(n, m)),
            false => println!(
                "{} is {}a multiple of {}.",
                n,
                if is_multiple(n, m) { "" } else { "not " },
                m
            ),
        },
        None => {
            println!("Please provide a function to use.");
        }
    }
}
