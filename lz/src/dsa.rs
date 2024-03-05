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
        None => {
            println!("Please provide a function to use.");
        }
    }
}
