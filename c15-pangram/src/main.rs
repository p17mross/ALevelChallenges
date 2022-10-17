use std::io::{self, BufRead, Write};

// Every letter to require
const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

/// Function to check if a string is a pangram
fn is_pangram(s: &str) -> bool {
    // Get lowercase of string
    let lower = s.to_lowercase();
    // Check all letters
    for c in ALPHABET.chars() {
        if !lower.contains(c) {return false}
    }
    return true
}

fn main() {
    // Input string
    let stdin = io::stdin();
    let mut s: String = "".to_string();
    print!("Enter a string: ");
    // Flush stdout
    io::stdout().flush().unwrap();
    stdin.lock().read_line(&mut s).unwrap();
    // Remove newline from end of string
    s = s[0..s.len() - 1].to_string();
    if is_pangram(&s) {
        println!("'{s}' is a pangram");
    }
    else {
        println!("'{s}' is not a pangram");
    }
}
