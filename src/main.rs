/*
TODO
1. Read in a text file
--- Accept the path to a text file via a cmd-line argument
ex. count_words.exe earth_to_the_moon.txt
--- Implement Error Handling
TODO
2. Count the number of times each word occurs
--- Parse text into individual words
Hint: split_whitespace() method
--- Ignore capitalization
TODO
3. Print a message with the most common words and how many times they appeared.
--- Keep track of how many times each unique word occurs
--- Print most common words and how many times they occur
*/
use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let _content = match env::args().nth(1) {
        Some(f) => match fs::read_to_string(f) {
            Ok(s) => s.to_lowercase(),
            Err(e) => {
                eprint!("Could not read file: {}", e);
                std::process::exit(1);
            }
        },
        None => {
            eprint!("Program requires an argument: <file path>");
            std::process::exit(2);
        }
    };

    //collect is an iterator, and we are turning it into a vector of string slices.
    let all_words = _content.split_whitespace().collect::<Vec<&str>>();

    let mut word_counts: HashMap<&str, u32> = HashMap::new();
    for word in all_words {
        *word_counts.entry(word).or_insert(0) += 1;
    }

    let mut top_count = 0u32;
    let mut top_words: Vec<&str> = Vec::new();
    for (key, val) in word_counts {
        match val.cmp(&top_count) {
            Ordering::Greater => {
                top_count = val;
                top_words.clear();
                top_words.push(key);
            }
            Ordering::Equal => {
                top_words.push(key);
            }
            Ordering::Less => {}
        }
    }

    println!("Top word(s) occurred {} times:", top_count);
    for word in top_words {
        println!("{}", word);
    }
}
