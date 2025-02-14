use std::env;
use std::fs;

fn main() {
    // parse args
    let pattern = env::args().nth(1).expect("no pattern given");
    let path = env::args().nth(2).expect("no path given");

    println!("Searching for '{}'", pattern);
    println!("In file '{}'", path);

    // read file
    let content = fs::read_to_string(path).expect("could not read file");
    println!("With text:\n{}", content);
}
