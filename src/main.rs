use cute::c;
use std::collections::HashSet;
use std::fs::read_to_string;

fn read_dictionary(filename: &str) -> HashSet<String> {
    let mut res = HashSet::new();

    for line in read_to_string(filename).unwrap().lines() {
        res.insert(line.to_string());
    }

    res
}

fn lev(str1: &str, str2: &str) -> usize {
    if str1.len() == 0 {
        return str2.len();
    } else if str2.len() == 0 {
        return str1.len();
    }

    if str1.chars().nth(0).unwrap() == str2.chars().nth(0).unwrap() {
        return lev(&str1[1..].to_string(), &str2[1..].to_string());
    }

    return 1 + [
        lev(&str1[1..].to_string(), str2),
        lev(str1, &str2[1..].to_string()),
        lev(&str1[1..].to_string(), &str2[1..].to_string()),
    ]
    .iter()
    .min()
    .unwrap();
}

fn get_num_corrections<'a>(
    word: &'a str,
    dictionary: &'a HashSet<String>,
) -> Vec<(&'a String, usize)> {
    c![(s, lev(word, s)), for s in dictionary]
}

fn main() {
    let dictionary = read_dictionary("words.txt");
    let input_string = "Helo mom, I love you!";

    let filtered_input: String = input_string
        .chars()
        .filter(|c| c.is_alphabetic() || *c == ' ')
        .collect();
    let filtered_input = filtered_input.to_lowercase();

    let words = filtered_input.split(" ").collect::<Vec<&str>>();

    println!("{input_string}");
    for word in words {
        if dictionary.contains(word) {
            continue;
        }

    }
}
