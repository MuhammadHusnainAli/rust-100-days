use rand::Rng;

fn main() {
    // This variable is used to store the user input
    let mut entered_number: String = String::new();

    // The random Number should be stored here
    let random_num: i32 = generate_random_number();

    // This Loop will continue until unless user entered the correct number
    while entered_number.trim() != random_num.to_string() {
        println!("Please Enter Your Number between 1 and 100 :");

        //Clear the previous input before entered the new number form user
        entered_number.clear();

        // Read the user input from the console
        std::io::stdin()
            .read_line(&mut entered_number)
            .expect("Failed to read line");

        // Convert the entered number from String to i32
        let integer_entered_number: i32 = entered_number
            .trim()
            .parse()
            .expect("Please enter a valid integer");

        // Check if the entered number is within the valid range
        if integer_entered_number < random_num {
            println!("Your number is too low!");
        } else if integer_entered_number > random_num {
            println!("Your number is too high!");
        } else {
            println!("Congratulations! You guessed the number: {}", random_num);
        }
    }
}

// Function to generate a random number between 1 and 100
fn generate_random_number() -> i32 {
    return rand::thread_rng().gen_range(1..=100);
}
