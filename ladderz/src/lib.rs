//! # ladderz
//!
//! Implementations of mathematical and technical concepts in Rust.
//!
//! # Installation
//!
//! To add the library crate to your project, add the following dependency under your `[dependencies]` section in your `Cargo.toml`:
//!
//! ```toml
//! ladderz = { git = "https://github.com/rzmk/ladderz", branch = "main" }
//! ```
//!
//! # Example
//!
//! ```rust
//!use ladderz::prealgebra::get_factors;
//!
//!let x: u32 = 10;
//!println!("The factors of {x} are {:?}.", get_factors(x));
//! ```
//!
//! ```console
//! The factors of 10 are {1, 5, 2, 10}.
//! ```
//!
//! For a more detailed example of how to use the `ladderz` crate, please see the [library example on GitHub](https://github.com/rzmk/ladderz#library-example).
//!
//! Choose a module to view its available functions.

/// Various pre-algebra implementations including factor pairs, factors, multiples, and more.
///
/// # Example
///
/// ```rust
/// use ladderz::prealgebra::get_factors;
///
/// let x: u32 = 10;
/// println!("The factors of {x} are {:?}.", get_factors(x));
/// ```
///
/// ```console
/// The factors of 10 are {1, 5, 2, 10}.
/// ```
///
pub mod prealgebra;

/// Various data structures and algorithms implementations.
///
/// # Example
///
/// ```rust
/// use ladderz::dsa::contains_duplicate;
///
/// let nums: Vec<i32> = vec![2, 3, 4, 2];
/// let result = contains_duplicate(nums.clone());
/// println!("The vector {:?} {} contain a duplicate.", &nums, if result { "does" } else { "does not" });
/// ```
///
/// ```console
/// The vector [2, 3, 4, 2] does contain a duplicate.
/// ```
///
pub mod dsa;
