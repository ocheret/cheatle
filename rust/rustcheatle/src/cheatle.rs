use regex::Regex;
use std::collections::HashSet;
use std::sync::Once;

pub fn do_something() {
    println!("Doing something");
}

// Regular expression for the entire alphabet
const ALPHABET: &str = "[abcdefghijklmnopqrstuvwxyz]";

// The bytes of the entire dictionary as a single string
static RAW_DICTIONARY: &str = include_str!("./fivechars.txt");

// Vector of all of the words in the dictionary
static mut WORDS: Vec<&str> = Vec::new();

static START: Once = Once::new();

#[derive(Debug)]
pub struct Cheatle {
    remaining: [String; 5],
    required: HashSet<char>,
    indices: Vec<usize>,
}

impl Cheatle {
    pub fn new() -> Self {
        unsafe {
            START.call_once(|| {
                WORDS = RAW_DICTIONARY.trim().split("\n").collect();
            });
        }
        let words_len = unsafe { WORDS.len() };
        Cheatle {
            remaining: [
                ALPHABET.to_string(),
                ALPHABET.to_string(),
                ALPHABET.to_string(),
                ALPHABET.to_string(),
                ALPHABET.to_string(),
            ],
            required: HashSet::<char>::new(),
            indices: (0..words_len).collect(),
        }
    }

    pub fn reset(&mut self) {
        *self = Cheatle::new();
    }

    pub fn remove_letter(&mut self, pos: usize, letter: char) {
        self.remaining[pos] = self.remaining[pos].replace(&letter.to_string(), "");
    }

    pub fn not_in_word(&mut self, letter: char) {
        for pos in 0..(self.remaining.len()) {
            self.remove_letter(pos, letter);
        }
    }

    pub fn placed_in_word(&mut self, pos: usize, letter: char) {
        self.remaining[pos] = letter.to_string();
        if self.required.contains(&letter) {
            self.required.remove(&letter);
        }
    }

    pub fn misplaced_in_word(&mut self, pos: usize, letter: char) {
        self.remove_letter(pos, letter);
        self.required.insert(letter);
    }

    fn has_required(&self, word: &str) -> bool {
        for c in &self.required {
            if !word.contains(&(*c).to_string()) {
                return false;
            }
        }
        true
    }

    pub fn filter(&mut self) {
        let expr = self.remaining.join("");
        let prog = Regex::new(&expr).unwrap();
        let mut filtered = Vec::<usize>::new();
        // XXX - can't seem to do this with ...iter().filter()...
        for i in &self.indices {
            unsafe {
                let w = WORDS[*i];
                if self.has_required(w) && prog.is_match(w) {
                    filtered.push(*i);
                }
            }
        }
        self.indices = filtered;
    }

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
