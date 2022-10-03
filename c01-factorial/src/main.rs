fn factorial_loop(n: u32) -> u32 {
    let mut factorial: u32 = 1;
    for i in 1..n + 1 {
        factorial *= i;
    }
    factorial
}

fn factorial_recursive(n: u32) -> u32 {
    match n {
        0 => 1,
        _ => factorial_recursive(n - 1) * n
    }
}

fn main() {
    println!("{}", factorial_loop(10));
    println!("{}", factorial_recursive(10));
}
