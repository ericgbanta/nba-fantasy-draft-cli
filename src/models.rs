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

    pub fn total_players(&self) -> usize {
        self.players.len()
    }

    /// Returns positions that need to be filled, prioritized by need
    pub fn get_needed_positions(&self) -> Vec<Position> {
        let counts = self.get_position_counts();
        let min_count = counts.values().min().copied().unwrap_or(0);
        
        // Return all positions that have the minimum count
        let mut needed: Vec<Position> = Position::all()
            .into_iter()
            .filter(|pos| counts.get(pos).unwrap_or(&0) == &min_count)
            .collect();
        
        // Sort by position for consistency
        needed.sort_by_key(|pos| format!("{:?}", pos));
        needed
    }
}
