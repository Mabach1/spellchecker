use std::fs::read_to_string;
use std::collections::HashSet;

fn read_lines(filename: &str) -> HashSet<String> {
    let mut res = HashSet::new();

    for line in read_to_string(filename).unwrap().lines() {
        res.insert(line.to_string());
    }

    res
}

fn main() {
    let dictionary = read_lines("words.txt");

    if dictionary.contains("hello") {
        println!("containing word");
    }
}
