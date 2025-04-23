use std::io;

fn celsius_to_fahrenheit() {
    let mut celsius_str: String = String::new();
    println!("Celsius:");
    io::stdin().read_line(&mut celsius_str).expect("Failed to get celsius from stdin");
    let celsius: i32 = celsius_str.trim().parse().unwrap();
    let result: i32 = celsius*9/5+32;
    println!("Fahrenheit:");
    println!("{result}")
}

fn fahrenheit_to_celsius() {
    let mut fahrenheit_str: String = String::new();
    println!("Fahrenheit:");
    io::stdin().read_line(&mut fahrenheit_str).expect("Failed to get fahrenheit from stdin");
    let fahrenheit: i32 = fahrenheit_str.trim().parse().unwrap();
    let result: i32 = (fahrenheit-32)*5/9;
    println!("Celsius:");
    println!("{result}")
}

fn main() {
    println!("1 - Celsius to Fahrenheit");
    println!("2 - Fahrenheit to Celsius");

    let mut choice: String = String::new();
    io::stdin().read_line(&mut choice).expect("error");
    let choice = choice.trim();
    if choice == "1" {
        celsius_to_fahrenheit()
    } else {
        fahrenheit_to_celsius()
    }
}
