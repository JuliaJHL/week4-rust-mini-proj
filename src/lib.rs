use std::fs::File;
use std::io::prelude::*;

// write a function that takes two strings and returns the levenstein distance between them
pub fn levenstein_distance(s1: &str, s2: &str) -> usize {
    let mut dp = vec![vec![0; s2.len() + 1]; s1.len() + 1];
    for i in 0..dp.len() {
        dp[i][0] = i;
    }
    for j in 0..dp[0].len() {
        dp[0][j] = j;
    }
    for i in 1..dp.len() {
        for j in 1..dp[0].len() {
            let cost = if s1.chars().nth(i - 1).unwrap() == s2.chars().nth(j - 1).unwrap() {
                0
            } else {
                1
            };
            dp[i][j] = std::cmp::min(
                dp[i - 1][j] + 1,
                std::cmp::min(dp[i][j - 1] + 1, dp[i - 1][j - 1] + cost),
            );
        }
    }
    dp[s1.len()][s2.len()]
}

// write a function that takes a string and a lists of strings and returns the string with the lowest levenstein distance
pub fn closest(s: &str, list: Vec<String>) -> String {
    let mut closest = String::new();
    let mut lowest = 0;
    for s2 in list {
        let levenstein = levenstein_distance(s, &s2);
        if levenstein < lowest || lowest == 0 {
            lowest = levenstein;
            closest = s2;
        }
    }
    closest
}

// write a function that reads a txt file, splits each line and choose the first word, and returns a list of words
pub fn read_txt(txt_file: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut file = File::open(txt_file).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    for line in contents.lines() {
        let mut line = line.split_whitespace();
        words.push(line.next().unwrap().to_string().to_lowercase());
    }
    words
}
