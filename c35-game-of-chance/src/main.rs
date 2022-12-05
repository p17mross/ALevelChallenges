use std::{io::{self, Write, BufRead}};

use rand::Rng;

/// A list of all prime numbers under 30.\
/// This is simpler than checking numbers for primality as all numbers that need prime-checking are under 30.
const PRIMES_UNDER_30: [u64; 10] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29
];

/// Calculates the reward given the user's wager, the user's guess and the result.
fn calculate_reward(wager: u64, guess: u64, result: u64) -> u64 {
    // Calculate multiplier
    let mut multiplier = 1;

    if guess != result {return 0}

    // If r is even, 2x multiplier
    if guess % 2 == 0 {multiplier *= 2};
    // If r is a multiple of 10, 3x multiplier
    if guess % 10 == 0 {multiplier *= 3};
    // If r is prime, 5x multiplier
    if PRIMES_UNDER_30.contains(&guess) {multiplier *= 5};
    // If r is less than 5, 2x multiplier
    if guess < 5 {multiplier *= 2};

    // Calculate final reward
    wager * multiplier
}

/// Reads a positive number from stdin given an upper bound and a prompt describing 
fn read_u64(prompt: &str, upper: u64) -> Result<u64, std::io::Error> {
        // Get stdin
        let stdin = io::stdin();

        // Loop until the user enters a correct number
        loop {
            print!("Enter {prompt} from 1 to {upper}: ");
            // Flush stdout to print prompt immediately
            io::stdout().flush()?;

            // Read a line from stdin
            let mut s: String = "".to_string();
            stdin.lock().read_line(&mut s)?;
            // Remove newline from end of string
            s = s[0..s.len() - 1].to_string();
            s = s.trim().to_string();

            // Parse the number
            let Ok(u) = s.parse::<u64>() else {
                println!("Invalid int");
                continue;
            };

            // Bound check the number
            if u == 0 || u > upper {
                println!("Number must be between 1 and {upper}");
                continue;
            }
            
            return Ok(u)
        }
}

fn main() {
    // Tell user starting balance
    let mut balance = 30;
    println!("Your balance is £{balance}");

    // Loop until user is bankrupt
    while balance != 0 {
        // Input guess and wager
        let guess = read_u64("a guess", 30).unwrap();
        let wager = read_u64("a wager", u64::min(balance, 10)).unwrap();

        // Subtract wager from balance
        balance -= wager;

        // Randomise result
        let result = rand::thread_rng().gen_range(1..=30);
        println!("The wheel landed on {result}!");

        // Calculate how much the user won
        let winnings = calculate_reward(wager, guess, result);
        balance += winnings;
        // Tell the user the result
        match winnings {
            0 => print!("You lose. "),
            _ => print!("You win £{winnings}! "),
        }
        // Tell the user their balance
        println!("Your balance is now £{balance}");
    }

    println!("You're bankrupt :(");
}
