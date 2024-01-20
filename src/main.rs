use cute::c;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::vec;

fn read_dictionary(filename: &str) -> Vec<String> {
    let mut res = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        res.push(line.to_string());
    }
    res
}

fn make_dictionary(lines: &Vec<String>) -> HashSet<String> {
    let mut res = HashSet::new();
    for line in lines {
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

fn wagner_fisher(str1: &str, str2: &str) -> u64 {
    let n = str1.len();
    let m = str2.len();

    let mut dp = vec![0u64; (n + 1) * (m + 1)];

    for i in 0..=n {
        for j in 0..=m {
            if i == 0 {
                dp[j] = j as u64;
            } else if j == 0 {
                dp[i * (m + 1)] = i as u64;
            } else if str1.chars().nth(i - 1).unwrap() == str2.chars().nth(j - 1).unwrap() {
                dp[i * (m + 1) + j] = dp[(i - 1) * (m + 1) + (j - 1)];
            } else {
                dp[i * (m + 1) + j] = 1u64
                    + [
                        dp[i * (m + 1) + (j - 1)],
                        dp[(i - 1) * (m + 1) + j],
                        dp[(i - 1) * (m + 1) + (j - 1)],
                    ]
                    .iter()
                    .min()
                    .unwrap();
            }
        }
    }

    *dp.last().unwrap()
}

fn get_num_corrections<'a>(word: &'a str, dictionary: &'a HashSet<String>) -> Vec<(&'a String, u64)> {
    c![(s, wagner_fisher(word, s)), for s in dictionary]
}

fn write_suggestions(unknown_word: &str, dictionary: &HashSet<String>, num_suggestions: usize) {
    let mut moves = get_num_corrections(unknown_word, &dictionary);
    moves.sort_by(|a, b| a.1.cmp(&b.1));

    let suggestions = (&moves[0..num_suggestions]).to_vec();
    let suggestions: Vec<&String> = suggestions.into_iter().map(|x| x.0).collect();

    for suggestion in suggestions {
        println!("  {unknown_word} -> {suggestion}");
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

fn clean_word(string: &str) -> String {
    string
        .to_string()
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect()
}

fn error_output(input_string: &str, word: (&str, usize), dictionary: &HashSet<String>) {
    println!("{input_string}");

    for _ in 0..word.1 {
        print!(" ");
    }

    for _ in 0..clean_word(word.0).len() {
        print!("^");
    }

    println!(" unknown word, maybe try");
    write_suggestions(&clean_word(&word.0.to_lowercase()), &dictionary, 4);
    print!("\n");
}

fn main() {
    let dictionary = read_dictionary("words.txt");
    let dictionary = make_dictionary(&dictionary);

    let input_string = "Helo mom, I luve you!";

    let word_index = get_word_indices(input_string);

    for (word, index) in &word_index {
        let cleaned_word = clean_word(word).to_lowercase();

        if !dictionary.contains(&cleaned_word) {
            error_output(input_string, (word, *index), &dictionary)
        }
    }
}
