// import input/output
use std::io;

fn main() {
    // constant term of conversion ratio
    const TEMP_CONVERSION_RATIO: f32 = 5.0/9.0;

    // Prompt for user to enter a temperature to convert
    println!("Enter a temperature in Fahrenheit.");

    // Mutable String variable initialized to an empty string
    let mut temp_fahrenheit = String::new();

    // User input
    io::stdin()
        .read_line(&mut temp_fahrenheit)
        .expect("Failed to read number");

    // Conversion of string to floating point data type
    let temp_fahrenheit: f32 = temp_fahrenheit.trim().parse().expect("Please type a number");

    println!("the temperature is: {}", temp_fahrenheit);

    // Conversion of Fahrenheit to Celsios
    // C = 5/9 * (F -32)
    let temp_celsius: f32 = TEMP_CONVERSION_RATIO * (temp_fahrenheit - 32.0);

    println!("The temp in Celsius is: {}", temp_celsius);
        
}
