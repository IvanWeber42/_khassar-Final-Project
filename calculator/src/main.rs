enum Operation {
    Add {x: f64, y: f64},
    Subtract {x: f64, y: f64},
    Multiply {x: f64, y: f64},
    Divide {x: f64, y: f64},
}

fn calculate(operation: Operation) -> f64 {
    match operation {
        Operation::Add { x, y } => x + y,
        Operation::Subtract { x, y } => x - y,
        Operation::Multiply { x, y } => x * y,
        Operation::Divide { x, y } => x / y,
    }
}

fn main() {
    println!("Please enter the first floating-point number:");
    let x = get_number_from_user();

    println!("Please enter the second floating-point number:");
    let y = get_number_from_user();

    println!("Please enter the operation that you want to perform. Possible options are Add, Subtract, Multiply, and Divide.");
    let operation = get_operation_from_user(x, y);

    println!("The result of the operation is {}", calculate(operation));
}

fn get_number_from_user() -> f64 {
    let mut input = String::new();
    loop {
        std::io::stdin().read_line(&mut input).expect("Failed to read line");

        // Try to parse the input as a floating-point number
        match input.trim().parse::<f64>() {
            Ok(parsed_number) => {
                return parsed_number;
            }
            Err(_) => {
                println!("Invalid input. Please enter a floating-point number.");
                input.clear();
                continue;
            }
        }
    }
}

fn get_operation_from_user(x: f64, y: f64) -> Operation {
    let mut input = String::new();
    loop {
        std::io::stdin().read_line(&mut input).expect("Failed to read line");

        // Try to get a valid operation from the user
        match input.trim() {
            "Add" => return Operation::Add { x, y },
            "Subtract" => return Operation::Subtract { x, y },
            "Multiply" => return Operation::Multiply { x, y },
            "Divide" => return Operation::Divide { x, y },
            _ => {
                println!("Invalid input. Valid options are Add, Subtract, Multiply, and Divide.");
                input.clear();
                continue;
            }
        }
    }
}