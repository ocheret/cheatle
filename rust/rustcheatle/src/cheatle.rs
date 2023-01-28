use regex::Regex;
use std::collections::HashMap;
use std::sync::Once;

/// Regular expression for the entire alphabet
const ALPHABET: &str = "[abcdefghijklmnopqrstuvwxyz]";

/// The bytes of the entire dictionary as a single string
static RAW_DICTIONARY: &str = include_str!("./fivechars.txt");

/// Vector of all of the words in the dictionary
static mut WORDS: Vec<&str> = Vec::new();

/// Used to do a one-time initialization
static START: Once = Once::new();

/// Representation of a game
#[derive(Debug)]
pub struct Cheatle {
    remaining: [String; 5],  // Regex for remaining possibilities for each position
    required: HashMap<char, usize>, // Set of chars which must be somewhere in the word
    indices: Vec<usize>,     // Integer indices of filtered words

}

/// Retrieve a word by index from the statically initialize set of words
fn get_word(i: usize) -> &'static str {
    unsafe { WORDS[i] }
}

/// Setup the statically initialized set of words
fn setup_words() {
    unsafe {
        START.call_once(|| {
            WORDS = RAW_DICTIONARY.trim().split("\n").collect();
        });
    }
}

/// Get the length of the statically initialized word list
fn get_words_length() -> usize {
    unsafe { WORDS.len() }
}

impl Cheatle {
    /// Create a new game instance
    pub fn new() -> Self {
        setup_words();
        let words_len = get_words_length();
        Cheatle {
            remaining: [
                ALPHABET.to_string(),
                ALPHABET.to_string(),
                ALPHABET.to_string(),
                ALPHABET.to_string(),
                ALPHABET.to_string(),
            ],
            required: HashMap::<char, usize>::new(),
            indices: (0..words_len).collect(),
        }
    }

    /// Reset the game state. Not really different from just making a new game
    /// but we're trying to be consistent with the python and go versions.
    pub fn reset(&mut self) {
        *self = Cheatle::new();
    }

    /// Remove a letter from the remaining possibilities for one position
    pub fn remove_letter(&mut self, pos: usize, letter: char) {
        self.remaining[pos] = self.remaining[pos].replace(&letter.to_string(), "");
    }

    /// Remove a letter from the entire word's remaining possibilities
    pub fn not_in_word(&mut self, letter: char) {
        for pos in 0..(self.remaining.len()) {
            self.remove_letter(pos, letter);
        }
    }

    /// Set one position's remaining possibilities to exactly on letter.
    pub fn placed_in_word(&mut self, pos: usize, letter: char) {
        self.remaining[pos] = letter.to_string();
    }

    /// Remove a letter from a certain position but note that it must be in
    /// the word somewhere.
    pub fn misplaced_in_word(&mut self, pos: usize, letter: char) {
        self.remove_letter(pos, letter);
        if !self.required.contains_key(&letter) {
            self.required.insert(letter, 1);
        }
    }

    pub fn set_min_occurences(&mut self, letter: char, count: usize) {
        self.required.insert(letter, count);
    }

    /// Check if a word has all of the required letters somewhere
    fn has_required(&self, word: &str) -> bool {
        for (letter, count) in &self.required {
            let count_in_word = word.chars().filter(|c| c == letter).count();
            if *count > count_in_word {
                return false;
            }
        }
        true
    }

    /// Filter the remaining words according to the latest commands
    pub fn filter(&mut self) {
        let expr = self.remaining.join("");
        let prog = Regex::new(&expr).unwrap();
        let mut filtered = Vec::<usize>::new();
        // XXX - can't seem to do this with ...iter().filter()...
        for i in &self.indices {
            let w = get_word(*i);
            if self.has_required(w) && prog.is_match(w) {
                filtered.push(*i);
            }
        }
        self.indices = filtered;
    }

    /// Retrieve the remaining words
    pub fn get_words(&self) -> Vec<&str> {
        let mut result = Vec::<&str>::new();
        for i in &self.indices {
            unsafe {
                result.push(WORDS[*i]);
            }
        }
        result
    }
}
