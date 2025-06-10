extern crate prettytable;
extern crate serde;
extern crate serde_json;
mod models;

use models::Team;
use prettytable::{Cell, Row, Table};
use std::collections::BTreeMap;
use std::io;

fn main() {
    let json_data = include_str!("../data/teams.json");
    let parsed: serde_json::Value = serde_json::from_str(json_data).expect("Error parsing JSON");

    // Choose draft style
    println!("Choose draft style: ");
    println!("1. Set order");
    println!("2. Snake");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim();

    if choice == "1" {
        println!("You chose 'Set order' draft style");
    } else if choice == "2" {
        println!("You chose 'Snake' draft style");
    } else {
        println!("Invalid choice");
        return;
    }

    // Display every team where nbaFranchise is true
    let teams: Vec<Team> = serde_json::from_value(parsed["response"].clone()).unwrap();

    let mut nba_teams: Vec<Team> = teams
        .into_iter()
        .filter(|team| team.nba_franchise == Some(true))
        .filter(|team| team.all_star == Some(false))
        .collect();

    // Assign new ids based on order
    let mut new_id = 1;
    for team in &mut nba_teams {
        team.display_id = Some(new_id);
        new_id += 1;
    }

    let mut divisions: BTreeMap<String, Vec<&Team>> = BTreeMap::new();

    for team in &nba_teams {
        if let Some(leagues) = &team.leagues {
            if let Some(division) = leagues["standard"]["division"].as_str() {
                divisions
                    .entry(division.to_string())
                    .or_insert(Vec::new())
                    .push(&team);
            }
        }
    }

    let mut table = Table::new();

    // Add the division headers to the table
    table.add_row(Row::new(
        divisions
            .keys()
            .map(|division| Cell::new(division))
            .collect(),
    ));

    // Find the division with the maximum number of teams
    let max_teams = divisions
        .values()
        .map(|teams| teams.len())
        .max()
        .unwrap_or(0);

    // For each row up to max_teams
    for i in 0..max_teams {
        let mut row = Vec::new();

        // For each division
        for teams in divisions.values() {
            // If the division has a team in the current row, add it to the row, otherwise add an empty cell
            if i < teams.len() {
                row.push(Cell::new(&format!(
                    "{}: {}",
                    teams[i].display_id.unwrap(),
                    teams[i].name
                )));
            } else {
                row.push(Cell::new(""));
            }
        }

        table.add_row(Row::new(row));
    }

    // Print the table
    table.printstd();

    // Have a player select one team they want to draft for
    println!("Select a team by entering its ID: ");
    let mut team_choice = String::new();
    io::stdin().read_line(&mut team_choice).unwrap();
    let team_choice: u32 = team_choice
        .trim()
        .parse()
        .expect("Please enter a valid number");

    match nba_teams
        .iter()
        .find(|&team| team.display_id == Some(team_choice))
    {
        Some(team) => println!("You chose to draft for: {}", team.name),
        None => println!("Invalid team choice"),
    }
}
