/// A program to convert temperatures between Celsius, Fahrenheit, and Kelvin.
/// prompts the user for type of conversion and temperature value,
/// then performs the conversion and displays the result.

fn main() {
    loop {
        println!(
            "{}",
            String::from("\nTemperature Conversion\n")
                + "Choose the type to convert from:"
                + "\n\t1. Celsius"
                + "\n\t2. Fahrenheit"
                + "\n\t3. Kelvin"
        );
        let choice = get_string("Enter your choice (1-3): ");
        match choice.as_str() {
            "1" => {
                let celsius = get_string("Enter temperature in Celsius: ");
                if let Ok(temp) = celsius.parse::<f64>() {
                    let fahrenheit = temp * 9.0 / 5.0 + 32.0;
                    let kelvin = temp + 273.15;
                    println!("{}°C is {:.2}°F and {:.2}K", temp, fahrenheit, kelvin);
                } else {
                    println!("Invalid input for Celsius.");
                }
            }
            "2" => {
                let fahrenheit = get_string("Enter temperature in Fahrenheit: ");
                if let Ok(temp) = fahrenheit.parse::<f64>() {
                    let celsius = (temp - 32.0) * 5.0 / 9.0;
                    let kelvin = celsius + 273.15;
                    println!("{}°F is {:.2}°C and {:.2}K", temp, celsius, kelvin);
                } else {
                    println!("Invalid input for Fahrenheit.");
                }
            }
            "3" => {
                let kelvin = get_string("Enter temperature in Kelvin: ");
                if let Ok(temp) = kelvin.parse::<f64>() {
                    let celsius = temp - 273.15;
                    let fahrenheit = celsius * 9.0 / 5.0 + 32.0;
                    println!("{}K is {:.2}°C and {:.2}°F", temp, celsius, fahrenheit);
                } else {
                    println!("Invalid input for Kelvin.");
                }
            }
            _ => {
                println!("Invalid choice, please select between 1-3.");
            }
        }
        // Prompt to continue or exit
        println!("\nWould you like to perform another conversion?");
        let choice = get_string("Press Enter to continue or type 'exit' to quit: ");
        if choice.to_lowercase() == "exit" {
            println!("Exiting the program.");
            break;
        }
    }
}

/// Prompts the user for input
fn get_string(prompt: &str) -> String {
    use std::io::{self, Write};

    print!("{}", prompt);
    io::stdout().flush().unwrap(); // Ensure the prompt is printed before reading input

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string() // Return input without trailing whitespace
}
