use std::env;

fn main() {
    let pattern = env::args().nth(1).expect("no pattern given");
    let path = env::args().nth(2).expect("no path given");

    println!("Searching for '{}'", pattern);
    println!("In file '{}'", path);
}
