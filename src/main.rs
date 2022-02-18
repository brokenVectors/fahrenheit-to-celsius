use std::io;

fn fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
    // (F − 32) × 5/9 = C
    return (fahrenheit - 32.0) * (0.555)
}

fn celsius_to_fahrenheit(celsius: f32) -> f32 {
    // (C × 9/5) + 32 = F
    return celsius * 1.8 + 32.0
}

fn clear_terminal() {
    println!("\x1b[2J");
}

fn main() {
    clear_terminal();
    let mut converting_to_celsius = true;
    {
        let mut user_input = String::new();
        println!("Would you like to:\n\t[1] Convert to Celsius?\n\t[2] Convert to Fahrenheit?");
        io::stdin().read_line(&mut user_input).expect("Failed to read user input");
        user_input = String::from(user_input.trim());
        if user_input == "1" {
            converting_to_celsius = true;
        }
        else {
            converting_to_celsius = false;
        }
    }
    
    if converting_to_celsius {
        println!("Enter a temperature in fahrenheit.");
    }
    else {
        println!("Enter a temperature in celsius.");
    }
    
    loop {
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read user input");
        user_input = String::from(user_input.trim());
        if user_input == "quit" {
            break;
        }
        let temperature = match user_input.trim().parse::<f32>() {
            Ok(i) => i,
    
            Err(_) => continue,
        };
        if converting_to_celsius {
            println!("{}°F = {}°C", temperature, fahrenheit_to_celsius(temperature));
        }
        else {
            println!("{}°C = {}°F", temperature, celsius_to_fahrenheit(temperature));
        }
        
    }
    
}
