use std::io;

const READLINE_ERROR: &str = "Failed to read line";
const DIVIDEBYZERO_ERROR: &str = "Tried to divide by zero.";

fn calculator() {
    let mut number1str: String = String::new();
    let mut number2str: String = String::new();

    // Ask for input 1
    println!("Input number #1!");
    io::stdin().read_line(&mut number1str).expect(READLINE_ERROR);

    // Ask for input 2
    println!("Input number #2!");
    io::stdin().read_line(&mut number2str).expect(READLINE_ERROR);

    // Parse strings as signed 32-bit integers
    let number1: f64 = number1str.trim().parse::<f64>().unwrap();
    let number2: f64 = number2str.trim().parse::<f64>().unwrap();

    // Ask for operation type
    let mut operation: String = String::new();
    println!("Provide operation type! (\"+\", \"-\", \"/\", \"*\", \"^\")");
    io::stdin().read_line(&mut operation).expect(READLINE_ERROR);

    // Trim to remove any whitespace
    let operation: &str = operation.trim();

    if operation == "+" {
        let result: f64 = number1+number2;
        println!("{number1} + {number2} = {result}");
    } else if operation == "-" {
        let result: f64 = number1-number2;
        println!("{number1} - {number2} = {result}");
    } else if operation == "/" {
        if number1 == 0.0 || number2 == 0.0 {
            println!("{DIVIDEBYZERO_ERROR}");
            return
        }
        let result: f64 = number1/number2;
        println!("{number1} / {number2} = {result}");
    } else if operation == "*" {
        let result: f64 = number1*number2;
        println!("{number1} * {number2} = {result}");
    } else if operation == "^" {
        let result: f64 = number1.powi(number2 as i32);
        println!("{number1} ^ {number2} = {result}");
    } else {
        println!("Unknown operation.");
    }
    println!("");
}

fn main() {
    loop {
        calculator();
    }
}
