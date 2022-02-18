use std::io;

fn fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
    // (F − 32) × 5/9 = C

    return (fahrenheit - 32.0) * (0.555)
}
fn main() {
    println!("Enter a temperature in fahrenheit.");
    loop {
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read user input");
        user_input = String::from(user_input.trim());
        if user_input == "quit" {
            break;
        }
        let temp_fahrenheit = match user_input.trim().parse::<f32>() {
            Ok(i) => i,
    
            Err(_) => continue,
        };
        println!("{}°F = {}°C", temp_fahrenheit, fahrenheit_to_celsius(temp_fahrenheit));
    }
    
}
