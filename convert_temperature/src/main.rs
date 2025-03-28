// Convert from Fahrenheit to Celsius
fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

// Convert from Celsius to Fahrenheit
fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * (9.0 / 5.0) + 32.0
}

fn main() {
    println!("What is the value of 50 degrees Fahrenheit in Celsius? Answer: {} degrees Celsius", fahrenheit_to_celsius(50.0));
    println!("What is the value of 10 degrees Celsius in Fahrenheit? Answer: {} degrees Fahrenheit", celsius_to_fahrenheit(10.0));
}
