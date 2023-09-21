//! # ladderz
//!
//! Implementations of mathematical and technical concepts in Rust.
//!
//! # Subjects
//!
//! The modules for currently supported subjects are:
//!
//! - [pre_algebra] - Various pre-algebra implementations including factor pairs, factors, multiples, and more.
//!
//! # Example
//!
//! Here's an example of using the `ladderz` crate to get the factors and factor pairs of a positive integer in sorted order.
//! We'll assume you're using Bash as your terminal.
//!
//! First let's create a new Rust project and change into the project directory:
//!
//! ```bash
//! cargo new my_ladderz_project
//! cd my_ladderz_project
//! ```
//!
//! Then let's add the following to `Cargo.toml` under the `[dependencies]` section:
//!
//! ```toml
//! ladderz = { git = "https://github.com/rzmk/ladderz", branch = "main" }
//! ```
//!
//! Now in `src/main.rs` let's replace the contents with the following code:
//!
//! ```rust
//! use ladderz::pre_algebra::{get_factors, get_factor_pairs};
//! use std::env;
//!
//! fn main() {
//!     // Get user input as a Vec
//!     let args: Vec<String> = env::args().collect();
//!
//!     // Check if input was provided
//!     match args.get(1) {
//!         Some(_) => {
//!             match args[1].parse::<u32>() {
//!                 // Handle input that can be parsed as a u32
//!                 Ok(x) => {
//!                     // Convert the HashSet of factors of input x to a sorted Vec
//!                     let mut factors: Vec<u32> = get_factors(x).into_iter().collect::<Vec<u32>>();
//!                     factors.sort();
//!
//!                     // Convert the HashSet of factor pairs of input x to a sorted Vec
//!                     let mut factor_pairs: Vec<(u32, u32)> =
//!                         get_factor_pairs(x).into_iter().collect::<Vec<(u32, u32)>>();
//!                     factor_pairs.sort();
//!
//!                     // Print the results
//!                     println!("List of factors of {:?}: {:?}", x, factors);
//!                     println!("List of factor pairs of {:?}: {:?}", x, factor_pairs);
//!                 }
//!                 // Handle input that can't be parsed as a u32
//!                 Err(e) => println!("Error parsing input: {e}"),
//!             }
//!         }
//!         None => println!("No input provided."),
//!     }
//! }
//!
//! ```
//!
//! Now let's build the project's binary file so we can run it from the command line:
//!
//! ```bash
//! cargo build --release
//! ```
//!
//! Our runnable binary file should be located at the local path `./target/release/my_ladders_project` (or `./target/release/my_ladders_project.exe` for Windows). Let's run it with the positive integer `12` as input:
//!
//! ```bash
//! ./target/release/my_ladderz_project 12
//! ```
//!
//! If you have a `.exe` file instead, you can run it with:
//!
//! ```bash
//! ./target/release/my_ladderz_project.exe 12
//! ```
//!
//! The printed output should be:
//!
//! ```console
//! List of factors of 12: [1, 2, 3, 4, 6, 12]
//! List of factor pairs of 12: [(1, 12), (2, 6), (3, 4)]
//! ```
//!
//! Great! We've successfully used the `ladderz` crate to get the factors and factor pairs of a positive integer in sorted order.

/// Various pre-algebra implementations including factor pairs, factors, multiples, and more.
///
/// # Example
///
/// ```rust
/// use ladderz::pre_algebra::get_factors;
///
/// fn main() {
///     let x: u32 = 10;
///     println!("The factors of {x} are {:?}.", get_factors(x));
/// }
/// ```
///
/// ```console
/// The factors of 10 are {1, 5, 2, 10}.
/// ```
///
pub mod pre_algebra;
