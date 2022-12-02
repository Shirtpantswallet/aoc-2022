use std::env;
use std::fs;

fn main() {
   let contents = fs::read_to_string("./src/bin/day1/input.txt")
     .expect("Should have been able to read the file");
   println!("{contents}");
}
