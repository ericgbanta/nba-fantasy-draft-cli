use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Define the team structure as per the API response
#[derive(Debug, Deserialize)]
pub struct RawTeam {
    pub all_star: bool,
    pub city: String,
    pub code: String,
    pub id: i32,
    pub leagues: HashMap<String, LeagueInfo>,
    pub logo: String,
    pub name: String,
    pub nba_franchise: bool,
    pub nickname: String,
}

#[derive(Debug, Deserialize)]
pub struct LeagueInfo {
    pub conference: String,
    pub division: Option<String>,
}

// Define the desired team structure for our output
#[derive(Debug, Serialize)]
pub struct Team {
    pub city: String,
    pub code: String,
    pub id: i32,
    pub nickname: String,
    pub logo: String,
}
