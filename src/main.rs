use reqwest::Client;
use tokio::io::{self, AsyncBufReadExt, BufReader};
use tokio::time::sleep;
use std::time::Duration;
use std::io::Write;

async fn check_website_for_value(url: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client.get(url).send().await?;
    let body = response.text().await?;
    
    if body.contains(value) {
        println!("It's a valid flag.");
    } else {
        println!("It's an invalid flag.");
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let stdin = io::stdin();
    let mut handle = BufReader::new(stdin);

    loop {
        print!("\x1b]0;Flag Watcher\x07");
        let mut input = String::new(); 
        println!("Enter your flag:"); 
        handle.read_line(&mut input).await.expect("Failed to read input");

        let search_string = input.trim();
        let url = "https://raw.githubusercontent.com/MaximumADHD/Roblox-Client-Tracker/roblox/FVariables.txt";

        if let Err(e) = check_website_for_value(url, search_string).await { 
            eprintln!("Error: {}", e);
        }

        sleep(Duration::from_secs(2)).await;

        print!("{}[2J", 27 as char);
        print!("{}[H", 27 as char);
        std::io::stdout().flush().expect("Failed to clear the screen");
    }
}
