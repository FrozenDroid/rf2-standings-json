#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Outer {
    pub total_rounds: usize,
    pub standings: Vec<Standings>,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Standings {
    pub position: u32,
    pub car_number: u32,
    pub driver_name: String,
    pub full_team_name: String,
    pub car_class: String,
    pub total_points: u32,
    pub rounds: Vec<Round>,
}

#[derive(Debug, serde::Serialize)]
pub struct Round {
    pub points: u32,
}
