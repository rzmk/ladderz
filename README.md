# ladderz

The `ladderz` project is a collection of math and tech concepts implemented in code.

-   **[`ladderz` library](https://rzmk.github.io/ladderz/)** - A Rust library for running concept implementations
-   **[`lz` CLI](lz)** - A command line tool for running ladderz functions
-   **[notebooks](notebooks)** - Rust & Python Jupyter notebooks with concept exercises and solutions

> If you're looking for a more efficient implementation of a concept (e.g., for use in your programs), other resources may be more useful.

## Demos

### ladderz Library Demo

![ladderz-crate-demo](https://github.com/rzmk/ladderz/assets/30333942/ebebe82d-f2f7-4604-9ea8-641325c90997)
![ladderz-crate-demo-output](https://github.com/rzmk/ladderz/assets/30333942/e84b981a-91ca-40ed-af1b-d3115be848fd)

### `lz` CLI Demo

![lz-demo](https://github.com/rzmk/ladderz/assets/30333942/e4f18934-b2bd-45cc-a948-ee28422f19e2)

## CLI Example

The `lz` CLI tool can be used to run ladderz functions from the command line. Built with the [clap](https://clap.rs) crate, users may select a subject and concept to run on their given input.

First let's install the `lz` CLI tool using [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html):

```bash
cargo install --git https://github.com/rzmk/ladderz --branch main
```

You may read the help text for the `lz` CLI tool and its subcommands with a `-h` or `--help` flag. For example, to read the help text for the `prealgebra` subject run:

```bash
lz prealgebra -h
```

Now let's run the `lz` CLI tool with the `prealgebra` subject and `get-factors` concept as an example. We want to get the factors of the positive integer `12`:

```bash
lz prealgebra get-factors 12
```

The printed output may be:

```console
The factors of 12 are {2, 3, 6, 1, 4, 12}.
```

We can use the `--raw` flag to get the raw output of the function:

```bash
lz prealgebra get-factors 12 --raw
```

The printed raw output may be:

```console
{1, 2, 3, 4, 6, 12}
```

Note that the numbers may not be sorted since the `get_factors` function implemented in the ladderz library returns a `HashSet<u32>`.

## Library Example

Here's an example of using the `ladderz` crate to get the factors and factor pairs of a positive integer in sorted order.

First let's create a new Rust project and change into the project directory:

```bash
cargo new my_ladderz_project
cd my_ladderz_project
```

Then let's add the following to `Cargo.toml` under the `[dependencies]` section:

```toml
ladderz = { git = "https://github.com/rzmk/ladderz", branch = "main" }
```

Now in `src/main.rs` let's replace the contents with the following code:

```rust
use ladderz::prealgebra::{get_factors, get_factor_pairs};
use std::env;

fn main() {
    // Get user input as a Vec
    let args: Vec<String> = env::args().collect();

    // Check if input was provided
    match args.get(1) {
        Some(_) => {
            match args[1].parse::<u32>() {
                // Handle input that can be parsed as a u32
                Ok(x) => {
                    // Convert the HashSet of factors of input x to a sorted Vec
                    let mut factors: Vec<u32> = get_factors(x).into_iter().collect::<Vec<u32>>();
                    factors.sort();

                    // Convert the HashSet of factor pairs of input x to a sorted Vec
                    let mut factor_pairs: Vec<(u32, u32)> =
                        get_factor_pairs(x).into_iter().collect::<Vec<(u32, u32)>>();
                    factor_pairs.sort();

                    // Print the results
                    println!("List of factors of {:?}: {:?}", x, factors);
                    println!("List of factor pairs of {:?}: {:?}", x, factor_pairs);
                }
                // Handle input that can't be parsed as a u32
                Err(e) => println!("Error parsing input: {e}"),
            }
        }
        None => println!("No input provided."),
    }
}
```

Now let's build the project's binary file so we can run it from the command line:

```bash
cargo build --release
```

Our runnable binary file should be located at the local path `./target/release/my_ladders_project` (or `./target/release/my_ladders_project.exe` for Windows). Let's run it with the positive integer `12` as input:

```bash
./target/release/my_ladderz_project 12
```

If you have a `.exe` file instead, you can run it with:

```bash
./target/release/my_ladderz_project.exe 12
```

The printed output should be:

```
List of factors of 12: [1, 2, 3, 4, 6, 12]
List of factor pairs of 12: [(1, 12), (2, 6), (3, 4)]
```

We've successfully used the `ladderz` crate to get the factors and factor pairs of a positive integer in sorted order.

## Ideas

Not currently implemented, but ideas that may be useful:

-   Extend to:
    -   Algebra
    -   Precalculus
    -   Calculus
    -   Linear Algebra
    -   Statistics
    -   Discrete Math
    -   Bioinformatics
-   **ladderz-py** - A package for running implementations of mathematical concepts in Python
-   **ladderz-app** - An interactive cross-platform (web, desktop, mobile) app potentially including visualizations, practice problems, & a course-like structure (potentially in Flutter or Tauri with Next.js & Rust)
