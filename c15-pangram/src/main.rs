use std::io::{self, BufRead, Write};

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";
fn is_pangram(s: &str) -> bool {
    let lower = s.to_lowercase();
    for c in ALPHABET.chars() {
        if !lower.contains(c) {return false}
    }
    return true
}

fn main() {
    let stdin = io::stdin();
    let mut s: String = "".to_string();
    print!("Enter a string: ");
    io::stdout().flush().unwrap();
    stdin.lock().read_line(&mut s).unwrap();
    s = s[0..s.len() - 1].to_string();
    if is_pangram(&s) {
        println!("'{s}' is a pangram");
    }
    else {
        println!("'{s}' is not a pangram");
    }
}
