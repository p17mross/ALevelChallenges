// Provides read!() macro
#[macro_use]
extern crate text_io;

const SMALL_NUMS: [&str; 20] = [
    "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", 
    "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen"
];
const TENS: [&str; 10] = [
    "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];
const ILLIONS: [&str; 7] = [
    "thousand", "million", "billion", "trillion", "quadrillion", "quintillion", "sextillion"
];

fn ilog10(mut i: u64) -> u32 {
    let mut j = 0;
    while i >= 10 {
        j += 1;
        i /= 10;
    }
    j
}

fn get_name(i: i64) -> String {
    
    if i == 0 {return "zero".to_string()}

    let mut parts = vec![];

    if i < 0 {
        return "minus ".to_string() + &get_name(-i);
    }
    if i >= 1000 {
        for illion in (0..=(ilog10(i as u64) / 3 - 1)).rev() {
            let part = i / (10i64.pow(illion * 3 + 3)) % 1000;
            if part != 0 {
                parts.push(get_name(part) + " " + ILLIONS[illion as usize]);
            }
        }
    }

    let hundreds = (i % 1000) / 100;
    if hundreds != 0 {
        parts.push(SMALL_NUMS[hundreds as usize].to_string() + " hundred");
    }

    if i % 100 != 0 {
        let ones;
        if i % 100 < 20 {
            ones = i % 100;
        }
        else {
            ones = i % 10;
        }
        let tens = (i % 100) / 10;
        if i > 100 {
            parts.push("and".to_string());
        }

        parts.push((TENS[tens as usize].to_string() + " " + SMALL_NUMS[ones as usize]).trim().to_string());
    }

    if parts.len() == 0 {
        panic!("length of parts was 0");
    }
    if parts.len() == 1 {
        return parts[0].clone()
    }
    else {
        return parts.join(" ")
    }
}

fn main() {
    print!("Enter a number: ");
    let i: String = read!();
    let i = i.parse::<i64>().expect("Enter a valid i64");
    println!("Name of {i} is '{}'", get_name(i));
}
