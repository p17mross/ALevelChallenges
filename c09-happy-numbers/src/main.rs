//A function to check if a number is a happy number or not
fn is_happy(i: u64) -> bool {
    let mut next = i;
    //keep track of previousy visited numbers to detect loops
    let mut visited = vec![next.clone()];
    loop {
        //convert to string to get digits of number
        let s = next.to_string();
        //get chars (one digit per char)
        next = s.chars()
            .map(
                |c| {
                    let u = u64::from(c) - u64::from('0');
                    let p = u.pow(2);
                    p
                }
            ).sum();
        //check if number has reached 1
        if next == 1 {
            return true;
        }
        //check for a loop - if back to a previously visited number
        if visited.contains(&next) {
            return false;
        }
        //add current number to list
        visited.push(next.clone());
    }
}

fn main() {
    let mut i = 0;
    let mut found = 0;
    //find first 20
    println!("The first 20 happy numbers are:");
    while found < 20 {
        i += 1;
        if is_happy(i) {
            found += 1;
            println!("{i}");
        }
    }
}
