use crate::models::{Player, Team, TeamRoster};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct DraftPick {
    pub round: u32,
    pub pick: u32,
    pub overall: u32,
    pub team: Team,
    pub player: Option<Player>,
}

#[derive(Debug)]
pub struct Draft {
    pub style: DraftStyle,
    pub rounds: u32,
    pub teams: Vec<Team>,
    pub draft_order: Vec<Team>,
    pub picks: Vec<DraftPick>,
    pub available_players: Vec<Player>,
    pub team_rosters: HashMap<u32, TeamRoster>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DraftStyle {
    SetOrder,
    Snake,
}

impl Draft {
    pub fn new(style: DraftStyle, teams: Vec<Team>) -> Self {
        let mut team_rosters = HashMap::new();
        for team in &teams {
            if let Some(id) = team.display_id {
                team_rosters.insert(id, TeamRoster::new(team.clone()));
            }
        }
        
        Self {
            style,
            rounds: 12,
            teams,
            draft_order: Vec::new(),
            picks: Vec::new(),
            available_players: Vec::new(),
            team_rosters,
        }
    }

    pub fn load_players(&mut self, players: Vec<Player>) {
        self.available_players = players;
        // Sort by salary descending (best players first)
        self.available_players.sort_by(|a, b| b.salary.cmp(&a.salary));
    }

    pub fn set_draft_order(&mut self, user_team_id: Option<u32>, user_position: Option<u32>) {
        let mut draft_teams = self.teams.clone();
        let mut rng = thread_rng();

        if let (Some(team_id), Some(position)) = (user_team_id, user_position) {
            // Find and remove the user's team
            if let Some(user_team_index) = draft_teams
                .iter()
                .position(|t| t.display_id == Some(team_id))
            {
                let user_team = draft_teams.remove(user_team_index);

                // Shuffle the remaining teams
                draft_teams.shuffle(&mut rng);

                // Insert user's team at the specified position (convert to 0-based index)
                let insert_position = (position - 1) as usize;
                if insert_position <= draft_teams.len() {
                    draft_teams.insert(insert_position, user_team);
                } else {
                    draft_teams.push(user_team);
                }
            }
        } else {
            // Random order for all teams
            draft_teams.shuffle(&mut rng);
        }

        self.draft_order = draft_teams;
    }

    pub fn generate_picks(&mut self) {
        self.picks.clear();
        let mut overall_pick = 1;

        for round in 1..=self.rounds {
            let round_order = match self.style {
                DraftStyle::SetOrder => self.draft_order.clone(),
                DraftStyle::Snake => {
                    if round % 2 == 1 {
                        // Odd rounds: normal order
                        self.draft_order.clone()
                    } else {
                        // Even rounds: reverse order
                        let mut reversed = self.draft_order.clone();
                        reversed.reverse();
                        reversed
                    }
                }
            };

            for (pick_in_round, team) in round_order.iter().enumerate() {
                let pick = DraftPick {
                    round,
                    pick: (pick_in_round + 1) as u32,
                    overall: overall_pick,
                    team: team.clone(),
                    player: None,
                };
                self.picks.push(pick);
                overall_pick += 1;
            }
        }
    }

    pub fn get_team_picks(&self, team_id: u32) -> Vec<&DraftPick> {
        self.picks
            .iter()
            .filter(|pick| pick.team.display_id == Some(team_id))
            .collect()
    }

    /// Smart computer picking: prioritizes filling positions first, then picks best available
    pub fn computer_pick(&mut self, team_id: u32) -> Option<Player> {
        if self.available_players.is_empty() {
            return None;
        }

        let roster = self.team_rosters.get(&team_id)?;
        
        // Find which position the team needs most
        if let Some(needed_position) = roster.needs_position() {
            // Try to find best available player for that position
            if let Some(idx) = self.available_players
                .iter()
                .position(|p| p.position == needed_position) 
            {
                let player = self.available_players.remove(idx);
                if let Some(roster) = self.team_rosters.get_mut(&team_id) {
                    roster.add_player(player.clone());
                }
                return Some(player);
            }
        }
        
        // If no specific position needed or none available, pick best available player
        let player = self.available_players.remove(0);
        if let Some(roster) = self.team_rosters.get_mut(&team_id) {
            roster.add_player(player.clone());
        }
        Some(player)
    }

    pub fn get_roster(&self, team_id: u32) -> Option<&TeamRoster> {
        self.team_rosters.get(&team_id)
    }

    pub fn print_draft_order(&self) {
        println!("\n=== DRAFT ORDER ===");
        for (i, team) in self.draft_order.iter().enumerate() {
            println!("{}. {}", i + 1, team.name);
        }
    }

    pub fn print_team_picks(&self, team_id: u32) {
        let team_picks = self.get_team_picks(team_id);
        if let Some(first_pick) = team_picks.first() {
            println!("\n=== YOUR DRAFT PICKS ({}) ===", first_pick.team.name);
            for pick in team_picks {
                println!(
                    "Round {}: Pick {} (Overall #{})",
                    pick.round, pick.pick, pick.overall
                );
            }
        }
    }
}
