use std::collections::HashSet;
use std::fs::read_to_string;

fn read_dictionary(filename: &str) -> HashSet<String> {
    let mut res = HashSet::new();

    for line in read_to_string(filename).unwrap().lines() {
        res.insert(line.to_string());
    }

    res
}

fn main() {
    let dictionary = read_dictionary("words.txt");
    let input_string = "Helo mom, I love you!";
    //                        ^ mistake on purpose

    let filtered_input: String = input_string
        .chars()
        .filter(|c| c.is_alphabetic() || *c == ' ')
        .collect();
    let filtered_input = filtered_input.to_lowercase();

    let words = filtered_input.split(" ").collect::<Vec<&str>>();

    println!("{input_string}");
    for word in words {
        if !dictionary.contains(word) {
            println!("Unknown word: {word}");
        }
    }
}
