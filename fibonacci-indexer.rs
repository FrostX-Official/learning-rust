use std::io;

fn index_fibonacci(number: u32) -> u32 {
    if number < 2 {
        return number;
    }
    index_fibonacci(number-1)+index_fibonacci(number-2)
}

fn _main() { // is this bad?
    let mut number_str: String = String::new();

    // Ask for input 1
    println!("Input fibonacci number index:");
    io::stdin().read_line(&mut number_str).expect("Failed to read line");

    // Parse string as signed 32-bit integer
    let number: u32 = number_str.trim().parse::<u32>().unwrap_or(1);
    let result: u32 = index_fibonacci(number);
    println!("{result}");
}

fn main() {
    loop {
        _main();
    }
}
