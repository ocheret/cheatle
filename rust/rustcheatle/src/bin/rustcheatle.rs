use std::io::Write;

use rustcheatle::cheatle::Cheatle;
use std::io;

fn prompt() {
    print!("cheatle> ");
    io::stdout().flush().ok();
}

fn print_help() {
    println!("reset : start a new cheating session");
    println!("- <LETTERS> : letters not in solution");
    println!("-N <LETTER> : letter in solution but not in Nth position");
    println!("+N <LETTER> : letter is in Nth position");
    println!(">=N <LETTER>: letter occurs at least N time in word");
    println!("list : list remaining solutions");
    println!("help : this messsage");
}

fn main() {
    let mut chtl = Cheatle::default();
    prompt();
    let mut buf = String::new();
    while let Ok(count) = io::stdin().read_line(&mut buf) {
        if count == 0 {
            // A count of 0 means we have reached EOF
            break;
        }
        let line = buf.trim().to_lowercase();
        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.is_empty() || tokens[0].is_empty() {
            prompt();
            continue;
        }
        let command = tokens[0];
        if count == 0 {
            break;
        }
        match command {
            "reset" => chtl.reset(),
            "help" => print_help(),
            "list" => {
                chtl.filter();
                let words = chtl.get_words();
                println!("{}", words.join("\n"));
                println!("count: {}", words.len());
            },
            "-" => {
                if tokens.len() < 2 {
                    print_help();
                    continue;
                }
                for c in tokens[1].chars() {
                    chtl.not_in_word(c);
                }
            },
            "-1" | "-2" | "-3" | "-4" | "-5" => {
                // XXX - if a letter is misplaced more than 1 time in a single
                // guess then the letter must be repeated at least that many
                // times in the word. What's a good way to capture this here?
                if tokens.len() != 2 {
                    print_help();
                    continue;
                }
                let pos = command[1..].parse::<usize>().unwrap() - 1;
                chtl.misplaced_in_word(pos, tokens[1].chars().collect::<Vec<char>>()[0]);
            },
            "+1" | "+2" | "+3" | "+4" | "+5" => {
                if tokens.len() != 2 {
                    print_help();
                    continue;
                }
                let pos = command[1..].parse::<usize>().unwrap() - 1;
                chtl.placed_in_word(pos, tokens[1].chars().collect::<Vec<char>>()[0]);
            },
            ">=1" | ">=2" | ">=3" | ">=4" | ">=5" => {
                if tokens.len() != 2 {
                    print_help();
                    continue;
                }
                let count = command[2..].parse::<usize>().unwrap();
                let letter = tokens[1].chars().collect::<Vec<char>>()[0];
                chtl.set_min_occurences(letter, count);
            }
            _ => {
                println!("Invalid command");
                print_help();
            }
        };
        buf = String::new();
        prompt();
    }
    println!("\nBye");
}
