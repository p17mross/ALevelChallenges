/// A function to calculate the factorial of a number using a loop
fn factorial_loop(n: u32) -> u32 {
    // Set starting value
    let mut factorial: u32 = 1;
    // For every i 1 < i < n, multiply value by i
    for i in 1..n + 1 {
        factorial *= i;
    }
    // Return value
    factorial
}

/// A function to calculate the factorial of a number recursively
fn factorial_recursive(n: u32) -> u32 {
    match n {
        // Base case: factorial of 0 is 1
        0 => 1,
        // Recursive case: factorial of n is n times the factorial of n - 1
        _ => factorial_recursive(n - 1) * n
    }
}

fn main() {
    println!("Factorial of 5 computed with a loop is {}", factorial_loop(5));
    println!("Factorial of 5 computed recursively is {}", factorial_recursive(5));

    println!("Factorial of 10 computed with a loop is {}", factorial_loop(10));
    println!("Factorial of 10 computed recursively is {}", factorial_recursive(10));
}
