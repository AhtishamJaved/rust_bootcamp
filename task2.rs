use std::io;

enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn calculate(operation: Operation) -> f64 {
    match operation {
        Operation::Add(x, y) => x + y,
        Operation::Subtract(x, y) => x - y,
        Operation::Multiply(x, y) => x * y,
        Operation::Divide(x, y) => {
            if y != 0.0 {
                x / y
            } else {
                panic!("Division by zero!")
            }
        }
    }
}
//features
fn main() {
    println!("Enter the first number:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let first_number: f64 = input.trim().parse().expect("Invalid number");

    // Prompt the user to input the operation
    println!("Enter the operation (+, -, *, /):");
    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let operation = match input.trim() {
        "+" => Operation::Add,
        "-" => Operation::Subtract,
        "*" => Operation::Multiply,
        "/" => Operation::Divide,
        _ => {
            println!("Invalid operation");
            return;
        }
    };

    // Prompt the user to input the second number
    println!("Enter the second number:");
    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let second_number: f64 = input.trim().parse().expect("Invalid number");

    let operation_instance = operation(first_number, second_number);

    let result = calculate(operation_instance);

    println!("Result: {}", result);
}
