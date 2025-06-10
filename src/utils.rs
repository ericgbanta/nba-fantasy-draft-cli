use std::io;

pub fn get_user_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn get_number_input(prompt: &str, min: u32, max: u32) -> Option<u32> {
    loop {
        let input = get_user_input(prompt);

        // Check for "no" response
        let lower_input = input.to_lowercase();
        if lower_input == "no" || lower_input == "n" {
            return None;
        }

        // Try to parse and validate number
        if let Ok(num) = input.parse::<u32>() {
            if num >= min && num <= max {
                return Some(num);
            }
            println!("Please enter a number between {} and {}", min, max);
        } else {
            println!("Please enter a valid number or 'no'");
        }
    }
}

pub fn get_yes_no_input(prompt: &str) -> bool {
    loop {
        let input = get_user_input(prompt);
        match input.to_lowercase().as_str() {
            "yes" | "y" => return true,
            "no" | "n" => return false,
            _ => println!("Please enter 'yes' or 'no'"),
        }
    }
}
