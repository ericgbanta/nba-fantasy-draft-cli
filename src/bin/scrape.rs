use reqwest;
use scraper::{Html, Selector};
use serde_json;
use std::collections::HashMap;
use std::fs;

const URL: &str = "https://hoopshype.com/salaries/players/";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get(URL).await?;
    let body = resp.text().await?;
    let fragment = Html::parse_document(&body);

    let row_selector = Selector::parse("tbody > tr").unwrap();
    let name_selector = Selector::parse("td:nth-child(2) > a").unwrap();
    let value_selector = Selector::parse("td:nth-child(3)").unwrap();

    let mut data = HashMap::new();

    for row in fragment.select(&row_selector) {
        let name_element = row.select(&name_selector).next();
        let value_element = row.select(&value_selector).next();

        if let (Some(name_element), Some(value_element)) = (name_element, value_element) {
            let name = name_element.text().collect::<String>().trim().to_string();
            let value = value_element
                .value()
                .attr("data-value")
                .unwrap_or("")
                .to_string();

            data.insert(name, value);
        }
    }

    let json_data = serde_json::to_string(&data)?;
    fs::write("data/player_salaries.json", json_data)?;

    println!("Scraping completed and data written to 'data/player_salaries.json'");

    Ok(())
}
