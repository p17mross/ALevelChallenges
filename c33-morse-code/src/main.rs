use std::io::{self, BufRead, Write};

/// Mapping from morse code to plain text
const MORSE_CHARS: [(&str, &str); 36] = [
    (".-",    "a"),
    ("-...",   "b"),
    ("-.-.",  "c"),
    ("-..",   "d"),
    (".",     "e"),
    ("..-.",  "f"),
    ("--.",   "g"),
    ("....",  "h"),
    ("..",    "i"),
    (".---",  "j"),
    ("-.-",   "k"),
    (".-..",  "l"),
    ("--",    "m"),
    ("-.",    "n"),
    ("---",   "o"),
    (".--.",  "p"),
    ("--.-",  "q"),
    (".-.",   "r"),
    ("...",   "s"),
    ("-",     "t"),
    ("..-",   "u"),
    ("...-",  "v"),
    (".--",   "w"),
    ("-..-",  "x"),
    ("-.--",  "y"),
    ("--..",  "z"),
    (".----", "1"),
    ("..---", "2"),
    ("...--", "3"),
    ("....-", "4"),
    (".....", "5"),
    ("-....", "6"),
    ("--...", "7"),
    ("---..", "8"),
    ("----.", "9"),
    ("-----", "0")
];

/// Converts morse code to plain text
fn from_morse_code(s: &str) -> Result<String, String> {
    let mut plain = String::new();
    // Loop over all words
    'words: for word in s.split("|") {
        // ignore empty words
        if word == "" {continue 'words}
        // Loop over all chars in a word
        'chars_in_word: for morse_char in word.split(" ") {
            // Ignore empty strings
            if morse_char == "" {continue 'chars_in_word}
            // Find the char in `MORSE_CHARS`
            for (key, c) in MORSE_CHARS {
                if morse_char == key {
                    // Add the plaintext char to `plain`
                    plain += c;
                    continue 'chars_in_word;
                }
            }
            // Char not found
            return Err(format!("Unknown morse code character '{morse_char}'"));
        }
        // Add space between words
        plain += " ";
    }
    // trim last ' '
    plain = plain[0..plain.len() - 1].to_string();
    Ok(plain)
}

/// Converts plain text to morse code
fn to_morse_code(s: &str) -> Result<String, String> {
    let mut morse = String::new();
    // Loop over all words
    'words: for word in s.split(" ") {
        // ignore empty words
        if word == "" {continue 'words}
        // Loop over all chars in a word
        'chars_in_word: for plain_char in word.chars() {
            // Find the char in `MORSE_CHARS`
            for (key, c) in MORSE_CHARS {
                // Add the morse code char to `morse`
                if plain_char.to_string().to_lowercase() == c {
                    morse += key;
                    morse += " ";
                    continue 'chars_in_word;
                }
            }
            // Char not found
            return Err(format!("'{plain_char}' has no morse code representation"));
        }
        // add "| " between words
        morse += "| ";
    }
    // trim spaces or '|'s at end of string
    morse = morse.trim_end_matches(|x|x == '|' || x == ' ').to_string();
    Ok(morse)
}

fn main() {
    // Lock stdin
    let stdin = io::stdin();
    // Get iter over lines of stdin
    let mut lines = stdin.lock().lines();


    print!("Do you want to convert to or from morse code? type 'to' or 'from': ");
    // Flush stdout to print text immediately
    io::stdout().flush().unwrap();

    match lines.next().unwrap().unwrap().as_str() {
        // Convert from morse to plain text
        "from" => {
            print!("Enter ciphertext: ");
            // Flush stdout to print text immediately
            io::stdout().flush().unwrap();

            let morse_text: String = lines.next().unwrap().unwrap();
            let plaintext = from_morse_code(&morse_text);

            match plaintext {
                Ok(plaintext) => println!("{plaintext}"),
                Err(e) => println!("There was an error converting your text: {e}"),
            }
        },
        // Convert from plain text to morse
        "to" => {
            print!("Enter plaintext: ");
            // Flush stdout to print text immediately
            io::stdout().flush().unwrap();


            let plaintext: String = lines.next().unwrap().unwrap();
            let morse_text = to_morse_code(&plaintext);

            match morse_text {
                Ok(morse_text) => println!("{morse_text}"),
                Err(e) => println!("There was an error converting your text: {e}"),
            }
        },
        // Error if anything else was typed
        _ => {
            println!("Invalid option");
            return
        }
    }
}
