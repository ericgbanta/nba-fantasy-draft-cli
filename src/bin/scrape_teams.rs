mod models;
extern crate dotenv;
extern crate reqwest;
extern crate serde_json;

use dotenv::dotenv;
use models::{RawTeam, Team};
use std::env;
use std::fs;

fn main() {
    dotenv().ok(); // Load environment variables from .env file

    let api_key = env::var("RAPIDAPI_KEY").expect("RAPIDAPI_KEY must be set in .env");
    let url = "https://api-nba-v1.p.rapidapi.com/teams";

    let client = reqwest::blocking::Client::new();

    let response = client
        .get(url)
        .header("X-RapidAPI-Key", &api_key)
        .header("X-RapidAPI-Host", "api-nba-v1.p.rapidapi.com")
        .send()
        .expect("Failed to send request");

    if response.status().is_success() {
        let response_json: serde_json::Value = response.json().expect("Failed to parse JSON");
        let json_str =
            serde_json::to_string_pretty(&response_json).expect("Failed to convert to string");

        fs::write("data/teams.json", json_str).expect("Unable to write file");
        println!("Saved teams data to data/teams.json");
    } else {
        println!("Error: {}", response.status());
    }
}
