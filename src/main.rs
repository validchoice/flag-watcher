use reqwest::Client;
use tokio::io::{self, AsyncBufReadExt, BufReader};
use tokio::time::sleep;
use std::time::Duration;
use std::io::Write;

// Function to check if the given flag is present on the specified URL
async fn check_website_for_value(url: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client.get(url).send().await?;
    let body = response.text().await?;
    
    // Print a message if the flag is found on the website
    if body.contains(value) {
        println!("It's a valid flag.");
    } else {
        // Print a message if the flag is not found
        println!("It's an invalid flag.");
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let stdin = io::stdin();
    let mut handle = BufReader::new(stdin);

    loop {
        // Set the terminal window title to 'Flag Watcher'
        print!("\x1b]0;Flag Watcher\x07");

        let mut input = String::new(); 
        
        // Display instructions to the user
        println!("Note: Flags starting with 'Debug' are not supported. Please ensure your flag is spelled correctly.");
        println!("Enter flag:"); 
        println!();

        // Read the user's input
        handle.read_line(&mut input).await.expect("Failed to read input");

        let search_string = input.trim();
        let url = "https://raw.githubusercontent.com/MaximumADHD/Roblox-Client-Tracker/roblox/FVariables.txt";

        // Check if the input starts with "Debug"
        if search_string.starts_with("Debug") {
            // Inform the user that the flag starting with "Debug" is not supported
            println!("{} is not supported by Flag Watcher.", search_string);
        // Check if the input starts with 'F', 'D', or 'S' and has more than one character
        } else if (search_string.starts_with('F') || search_string.starts_with('D') || search_string.starts_with('S')) && search_string.len() > 1 {
            // Check the website for the specified flag
            if let Err(e) = check_website_for_value(url, search_string).await { 
                eprintln!("Error occurred while checking the flag: {}", e);
            }
        } else {
            // Inform the user if the flag is either not supported or unknown
            println!("{} is either not supported by Flag Watcher or is an unknown flag.", search_string);
        }

        // Wait for 2 seconds before clearing the screen
        sleep(Duration::from_secs(2)).await;

        // Clear the terminal screen
        print!("{}[2J", 27 as char);  // Clear the screen
        
        // Move the cursor to the top-left corner of the terminal
        print!("{}[H", 27 as char);  // Move cursor to home position
        
        std::io::stdout().flush().expect("Failed to clear the screen");
    }
}
