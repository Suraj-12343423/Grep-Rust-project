# Mini Grep — Rust

A simple command-line text search tool written in Rust.

This project is inspired by the basic functionality of the Unix `grep` command. It reads a text file line by line and prints the lines containing a given search word.

## Features

* Reads text from a file
* Searches for a specific word
* Prints matching lines
* Uses Rust's standard library
* Takes input through command-line arguments

## Project Structure

```text
mini-grep/
├── Cargo.toml
├── src/
│   └── main.rs
└── notes.txt
```

## Requirements

* Rust
* Cargo

Check your installation:

```bash
rustc --version
cargo --version
```

## Setup

Clone or create the project:

```bash
cargo new mini-grep
cd mini-grep
```

Then place your text file in the project root:

```text
mini-grep/
└── notes.txt
```

Example `notes.txt`:

```text
I am learning Rust.
Rust is a systems programming language.
I am also learning C.
Rust provides memory safety.
Go is another programming language.
```

## Run the Project

Build and run:

```bash
cargo run
```

The program can also be run directly:

```bash
cargo run -- Rust notes.txt
```

Here:

* `Rust` → search word
* `notes.txt` → file to search

## Example Output

```text
I am learning Rust.
Rust is a systems programming language.
Rust provides memory safety.
```

## How It Works

The program:

1. Reads command-line arguments using `std::env::args()`
2. Gets the search word and filename
3. Reads the file using `std::fs::read_to_string()`
4. Splits the file into lines using `.lines()`
5. Checks each line using `.contains()`
6. Prints matching lines

## Example Code

```rust
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let search_word = &args[1];
    let filename = &args[2];

    let content = fs::read_to_string(filename).unwrap();

    for line in content.lines() {
        if line.contains(search_word) {
            println!("{}", line);
        }
    }
}
```

## Concepts Learned

* `use`
* `std::env`
* `std::fs`
* `env::args()`
* `Vec<String>`
* Command-line arguments
* File reading
* `Result`
* `.unwrap()`
* `.lines()`
* `.contains()`
* `for` loops
* `if` statements
* `println!`

## Future Improvements

Possible next steps:

* Handle errors instead of using `.unwrap()`
* Make the search case-insensitive
* Support multiple search options
* Search multiple files
* Add tests
* Separate searching logic into functions
* Build a more complete `grep`-like CLI

## License

This project is for learning Rust and systems programming.

```

This README matches the **current Mini Grep stage** you're learning, rather than jumping ahead to the more advanced implementation.
```
