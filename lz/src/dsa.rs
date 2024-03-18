use clap::Subcommand;

#[derive(Subcommand)]
#[command(arg_required_else_help(true))]
pub enum Dsa {
    /// Returns true or false based on whether the vector has a duplicate.
    ///
    /// ## Example
    ///
    /// ### Input
    ///
    /// ```bash
    /// lz dsa contains-duplicate 1,2,3,2
    /// # Alternatively you may delimit the numbers with spaces:
    /// lz dsa contains-duplicate 1 2 3 2
    /// ```
    ///
    /// ### Output
    ///
    /// ```bash
    /// The vector [1, 2, 3, 2] does contain a duplicate.
    /// ```
    ///
    /// ## Raw Output (use `-r` or `--raw`)
    ///
    /// ```bash
    /// true
    /// ```
    ContainsDuplicate {
        /// The vector of numbers to detect whether it has a duplicate.
        #[arg(value_delimiter = ',', num_args = 1)]
        n: Vec<i32>,
        /// Whether or not to return the raw output.
        #[arg(short = 'r', long)]
        raw: bool,
    },
    /// Returns true or false based on whether string a is an anagram of string b.
    ///
    /// ## Example
    ///
    /// ### Input
    ///
    /// ```bash
    /// lz dsa is-anagram marc cram
    /// ```
    ///
    /// ### Output
    ///
    /// ```bash
    /// "marc" is an anagram of "cram".
    /// ```
    ///
    /// ## Raw Output (use `-r` or `--raw`)
    ///
    /// ```bash
    /// true
    /// ```
    IsAnagram {
        /// The first string to compare against.
        a: String,
        /// The second string to compare against.
        b: String,
        /// Whether or not to return the raw output.
        #[arg(short = 'r', long)]
        raw: bool,
    },
    /// Returns the indices of two numbers in a vector that sum to a target number.
    ///
    /// There is an assumption that there must always exist at least two numbers that
    /// sum to the target number in the given vector of numbers.
    ///
    /// ## Example
    ///
    /// ### Input
    ///
    /// ```bash
    /// lz dsa two-sum 1,2,3 5
    /// ```
    ///
    /// ### Output
    ///
    /// ```bash
    /// [1, 2]
    /// ```
    ///
    /// ## Raw Output (use `-r` or `--raw`)
    ///
    /// ```bash
    /// [1, 2]
    /// ```
    TwoSum {
        /// The vector of numbers as a comma-delimited string.
        nums: String,
        /// The number that two numbers from nums must sum to.
        target: i32,
        /// Whether or not to return the raw output.
        #[arg(short = 'r', long)]
        raw: bool,
    },
}

pub fn match_dsa(function: Option<Dsa>) {
    use ladderz::dsa::*;
    match function {
        Some(Dsa::ContainsDuplicate { n, raw }) => match raw {
            true => println!("{:?}", contains_duplicate(n)),
            false => {
                let result = contains_duplicate(n.clone());
                println!(
                    "The vector {:?} {} contain a duplicate.",
                    &n,
                    if result { "does" } else { "does not" }
                )
            }
        },
        Some(Dsa::IsAnagram { a, b, raw }) => match raw {
            true => println!("{:?}", is_anagram(a, b)),
            false => {
                let result = is_anagram(a.clone(), b.clone());
                println!(
                    "{:?} {} an anagram of {:?}.",
                    a,
                    if result { "is" } else { "is not" },
                    b,
                )
            }
        },
        Some(Dsa::TwoSum { target, nums, raw }) => {
            let nums_vec: Vec<i32> = nums
                .split(',')
                .map(|num| num.trim())
                .map(|num| num.parse::<i32>().unwrap())
                .collect();

            match raw {
                true => println!("{:?}", two_sum(nums_vec, target)),
                false => println!(
                    "The pair of indices of the two numbers that sum to {target} is: {:?}.",
                    two_sum(nums_vec, target)
                ),
            }
        }
        None => {
            println!("Please provide a function to use.");
        }
    }
}
