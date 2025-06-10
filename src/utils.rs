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
        match input.parse::<u32>() {
            Ok(num) if num >= min && num <= max => return Some(num),
            Ok(_) => println!("Please enter a number between {} and {}", min, max),
            Err(_) => {
                if input.to_lowercase() == "no" || input.to_lowercase() == "n" {
                    return None;
                }
                println!("Please enter a valid number or 'no'");
            }
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
