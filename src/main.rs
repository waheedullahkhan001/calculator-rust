fn main() {
    println!("Welcome to calculator");

    loop {
        let mut choice = String::new();

        println!("1. Addition");
        println!("2. Subtraction");
        println!("3. Multiplication");
        println!("4. Division");

        println!("Enter your choice below:");

        std::io::stdin()
                .read_line(&mut choice)
                .expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => 0,
        };

        match choice {
            1 => addition(),
            2 => subtraction(),
            3 => multiplication(),
            4 => division(),
            _ => println!("Invalid choice"),
        }
    }
    
}

fn get_input() -> (f32, f32) {
    let mut num1 = String::new();
    let mut num2 = String::new();

    println!("Enter first number:");
    std::io::stdin()
            .read_line(&mut num1)
            .expect("Failed to read line");

    println!("Enter second number:");
    std::io::stdin()
            .read_line(&mut num2)
            .expect("Failed to read line");

    let num1: f32 = match num1.trim().parse() {
        Ok(num) => num,
        Err(_) => 0.0,
    };

    let num2: f32 = match num2.trim().parse() {
        Ok(num) => num,
        Err(_) => 0.0,
    };

    (num1, num2)
}

fn addition() {
    let num1: f32;
    let num2: f32;
    
    (num1, num2) = get_input();

    println!("Result: {}", num1 + num2);
}

fn subtraction() {
    let num1: f32;
    let num2: f32;
    
    (num1, num2) = get_input();

    println!("Result: {}", num1 - num2);
}

fn multiplication() {
    let num1: f32;
    let num2: f32;
    
    (num1, num2) = get_input();

    println!("Result: {}", num1 * num2);
}

fn division() {
    let num1: f32;
    let num2: f32;
    
    (num1, num2) = get_input();

    println!("Result: {}", num1 / num2);
}

