#[macro_use]
extern crate text_io;

fn print_combinations(s: String, c: String) {
    //base case
    if c == "" {
        println!("{s}");
        return;
    }

    //store previous first chars so that the same thing is not printed multiple times
    let mut tried = "".to_string();
    for char in c.chars() {
        //skip char if it has been printed before
        if tried.contains(char) {
            continue;
        }
        //recurse
        print_combinations(s.clone() + &char.to_string(), c.replacen(char, "", 1));
        tried += &char.to_string();
    }
}

fn main() {
    print!("Enter a number: ");
    print_combinations("".to_string(), read!());
}
