use serde::{Deserialize, Serialize};

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
