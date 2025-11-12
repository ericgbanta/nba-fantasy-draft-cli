extern crate prettytable;
extern crate serde;
extern crate serde_json;
mod draft;
mod models;
mod utils; // Add this line

use draft::{Draft, DraftStyle};
use models::{Player, Position, Team};
use prettytable::{Cell, Row, Table};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::{BTreeMap, HashMap};
use utils::{get_number_input, get_user_input, get_yes_no_input}; // Add this line

fn load_players() -> Vec<Player> {
    let player_data = include_str!("../data/player_salaries.json");
    let salaries: HashMap<String, String> =
        serde_json::from_str(player_data).expect("Error parsing player salaries");

    let mut players = Vec::new();
    let positions = Position::all();
    let mut rng = thread_rng();

    for (name, salary_str) in salaries {
        // Parse salary, removing any commas
        let salary = salary_str.replace(",", "").parse::<u64>().unwrap_or(0);
        
        // Randomly assign a position for simplicity
        let position = positions.choose(&mut rng).unwrap().clone();
        
        players.push(Player {
            name,
            salary,
            position,
        });
    }

    players
}

fn main() {
    let json_data = include_str!("../data/teams.json");
    let parsed: serde_json::Value = serde_json::from_str(json_data).expect("Error parsing JSON");

    // Choose draft style using utils
    let choice =
        get_user_input("Choose draft style:\n1. Set order\n2. Snake\nEnter your choice (1 or 2):");

    // Store the draft style for later use
    let draft_style = match choice.as_str() {
        "1" => {
            println!("You chose 'Set order' draft style");
            DraftStyle::SetOrder
        }
        "2" => {
            println!("You chose 'Snake' draft style");
            DraftStyle::Snake
        }
        _ => {
            println!("Invalid choice");
            return;
        }
    };

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
                    .or_default()
                    .push(team);
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

    // Have a player select one team they want to draft for using utils
    let team_choice = match get_number_input(
        "Select a team by entering its ID:",
        1,
        nba_teams.len() as u32,
    ) {
        Some(choice) => choice,
        None => {
            println!("No team selected");
            return;
        }
    };

    let selected_team = match nba_teams
        .iter()
        .find(|&team| team.display_id == Some(team_choice))
    {
        Some(team) => {
            println!("You chose to draft for: {}", team.name);
            team.clone()
        }
        None => {
            println!("Invalid team choice");
            return;
        }
    };

    // Ask if user wants to set their draft position using utils
    let want_to_set_position =
        get_yes_no_input("Do you want to set the order of where your team drafts? (yes/no):");

    let user_position = if want_to_set_position {
        get_number_input(
            &format!(
                "Enter a number from 1-{} for your draft position:",
                nba_teams.len()
            ),
            1,
            nba_teams.len() as u32,
        )
    } else {
        println!("Random draft position will be assigned to all teams.");
        None
    };

    // Create and set up the draft
    let mut draft = Draft::new(draft_style, nba_teams);
    draft.set_draft_order(selected_team.display_id, user_position);
    draft.generate_picks();

    // Load players
    println!("\nLoading players...");
    let players = load_players();
    println!("Loaded {} players", players.len());
    draft.load_players(players);

    // Display results
    draft.print_draft_order();

    if let Some(team_id) = selected_team.display_id {
        draft.print_team_picks(team_id);
    }

    // Conduct the draft with computer picks
    println!("\n=== CONDUCTING DRAFT ===");
    println!("Computer teams will now make their picks based on team needs...\n");
    
    let num_picks = draft.picks.len();
    for i in 0..num_picks {
        let (team_id, team_name, overall) = {
            let pick = &draft.picks[i];
            (pick.team.display_id, pick.team.name.clone(), pick.overall)
        };
        
        if let Some(tid) = team_id {
            if let Some(player) = draft.computer_pick(tid) {
                println!("Pick #{}: {} selects {} ({:?}, ${})", 
                    overall, team_name, player.name, player.position, player.salary);
                draft.picks[i].player = Some(player);
            }
        }
    }

    // Display final rosters
    println!("\n=== FINAL ROSTERS ===");
    if let Some(team_id) = selected_team.display_id {
        if let Some(roster) = draft.get_roster(team_id) {
            println!("\nYour team ({}):", roster.team.name);
            let mut sorted_players = roster.players.clone();
            sorted_players.sort_by_key(|p| p.salary);
            sorted_players.reverse();
            
            for (i, player) in sorted_players.iter().enumerate() {
                println!("  {}. {} - {:?} (${:})", i + 1, player.name, player.position, player.salary);
            }
            
            let position_counts = roster.get_position_counts();
            println!("\n  Position breakdown:");
            for pos in Position::all() {
                println!("    {:?}: {}", pos, position_counts.get(&pos).unwrap_or(&0));
            }
        }
    }

    println!("\n=== DRAFT COMPLETE ===");
    println!("Total picks made: {}", draft.picks.len());
    println!("Draft style: {:?}", draft.style);
}
