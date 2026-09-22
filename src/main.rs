use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    let content = fs::read_to_string(filename).unwrap();

    for line in content.lines() {
        if line.contains(query) {
            println!("{}", line);
        }
    }
}