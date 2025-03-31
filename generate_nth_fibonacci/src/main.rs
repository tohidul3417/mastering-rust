// Generate the nth fibonacci number
fn generate_nth_fibonacci(n:u32) -> u128 {
    if n == 0 {
        return 0;
    }
    let mut a = 0;
    let mut b = 1;

    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp; // Store the new fibonacci in 'b'
    }
    b // Return nth fibonacci
}

fn main() {
    println!("What is the 3rd fibonacci number? Answer: {}", generate_nth_fibonacci(3));
    println!("What is the 5th fibonacci number? Answer: {}", generate_nth_fibonacci(5));
    println!("What is the 15th fibonacci number? Answer: {}", generate_nth_fibonacci(15));
    println!("What is the 20th fibonacci number? Answer: {}", generate_nth_fibonacci(20));
}
