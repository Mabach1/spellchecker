use cute::c;
use std::collections::HashSet;
use std::env::current_exe;
use std::fs::read_to_string;
use std::vec;

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

fn write_suggestions(unknown_word: &str, dictionary: &HashSet<String>) {
    println!("\nUnknown word: \"{}\", maybe try:", unknown_word);

    let mut moves = get_num_corrections(unknown_word, &dictionary);
    moves.sort_by(|a, b| a.1.cmp(&b.1));

    let suggestions = (&moves[0..=3]).to_vec();
    let suggestions: Vec<&String> = suggestions.into_iter().map(|x| x.0).collect();

    for suggestion in suggestions {
        println!("{unknown_word} -> {suggestion}");
    }
}

fn get_word_indices(input: &str) -> Vec<(&str, usize)> {
    let words: Vec<&str> = input.split_whitespace().collect();

    let mut word_positions = Vec::new();
    let mut current_position = 0;

    for word in &words {
        let start_position = input[current_position..]
            .find(word)
            .map(|pos| current_position + pos)
            .expect("Word not found in the remaining string");

        current_position = start_position + word.len();

        word_positions.push(start_position);
    }

    let mut res: Vec<(&str, usize)> = Vec::new();

    for (word, start_position) in words.iter().zip(word_positions.iter()) {
        res.push((word, *start_position));
    }

    res
}

fn draw_squiggly_lines(input: &Vec<(&str, usize)>, indices: &Vec<usize>) {
    for ((word, index), &index_value) in input.iter().zip(indices.iter().cycle()) {
        if *index == index_value {
            for _ in 0..clean_word(word).len() {
                print!("^");
            }
        } else {
            for _ in 0..clean_word(word).len() {
                print!(" ");
            }
        }

        for _ in 0..(word.len() - clean_word(word).len() + 1) {
            print!(" ");
        }
    }
}

fn clean_word(string: &str) -> String {
    string
        .to_string()
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect()
}

fn main() {
    let dictionary = read_dictionary("words.txt");
    let input_string = "Helo mom, I luve you!";

    let word_index = get_word_indices(input_string);

    let mut incorrect_indices: Vec<usize> = Vec::new();
    let mut incorrect_words: Vec<String> = Vec::new();

    for (word, index) in &word_index {
        let cleaned_word = clean_word(word).to_lowercase();

        if !dictionary.contains(&cleaned_word) {
            incorrect_indices.push(*index);
            incorrect_words.push(cleaned_word);
        }
    }

    println!("{input_string}");
    draw_squiggly_lines(&word_index, &incorrect_indices);
    println!("");

    // for word in words {
    //     if !dictionary.contains(word) {
    //         write_suggestions(word, &dictionary);
    //     }
    // }
}
