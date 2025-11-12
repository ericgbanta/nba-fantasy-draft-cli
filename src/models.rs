use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Team {
    #[serde(rename = "allStar")]
    pub all_star: Option<bool>,
    pub city: Option<String>,
    pub code: Option<String>,
    pub id: u32,
    pub display_id: Option<u32>,
    pub leagues: Option<serde_json::Value>,
    pub logo: Option<String>,
    pub name: String,
    #[serde(rename = "nbaFranchise")]
    pub nba_franchise: Option<bool>,
    pub nickname: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Position {
    PG, // Point Guard
    SG, // Shooting Guard
    SF, // Small Forward
    PF, // Power Forward
    C,  // Center
}

impl Position {
    pub fn all() -> Vec<Position> {
        vec![Position::PG, Position::SG, Position::SF, Position::PF, Position::C]
    }
}

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub salary: u64,
    pub position: Position,
}

#[derive(Debug, Clone)]
pub struct TeamRoster {
    pub team: Team,
    pub players: Vec<Player>,
}

impl TeamRoster {
    pub fn new(team: Team) -> Self {
        Self {
            team,
            players: Vec::new(),
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    pub fn position_count(&self, position: &Position) -> usize {
        self.players.iter().filter(|p| &p.position == position).count()
    }

    pub fn get_position_counts(&self) -> HashMap<Position, usize> {
        let mut counts = HashMap::new();
        for pos in Position::all() {
            counts.insert(pos.clone(), self.position_count(&pos));
        }
        counts
    }

    pub fn needs_position(&self) -> Option<Position> {
        // Find the position with the least players
        let counts = self.get_position_counts();
        Position::all()
            .into_iter()
            .min_by_key(|pos| counts.get(pos).unwrap_or(&0))
    }
}
