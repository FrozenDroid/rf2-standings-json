#![allow(dead_code)]

use std::collections::HashMap;

use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct Outer {
    #[serde(rename = "Class Overall")]
    pub class_overall: ClassOverall,
}

#[derive(Deserialize, Debug)]
pub struct ClassOverall {
    #[serde(rename = "Driver Standings")]
    pub driver_standings: DriverStandings,
    #[serde(rename = "Team Standings")]
    pub team_standings: TeamStandings,
}

#[derive(Deserialize, Debug)]
pub struct DriverStandings {
    pub drivers: HashMap<u32, Driver>,
    // rounds: Vec<Round>,
}

#[derive(Deserialize, Debug)]
pub struct Driver {
    pub name: String,
    #[serde(deserialize_with = "deserialize_hashmap")]
    pub positions: HashMap<usize, Option<u32>>,
    #[serde(deserialize_with = "deserialize_hashmap")]
    pub points: HashMap<usize, Option<u32>>,
    pub points_sum: u32,
    pub vehicles: String,
    pub vehicles_per_event: HashMap<u32, String>,
}

#[derive(Deserialize, Debug)]
pub struct TeamStandings {
    pub teams: HashMap<u32, Team>,
    // rounds: Vec<Round>,
}

#[derive(Deserialize, Debug)]
pub struct Team {
    pub team_name: String,
    #[serde(deserialize_with = "deserialize_drivers")]
    pub team_drivers: Vec<String>,
}

fn deserialize_vec_u32<'de, D>(deserializer: D) -> Result<Vec<u32>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let values: Vec<Value> = Vec::deserialize(deserializer)?;

    let mut numbers: Vec<u32> = Vec::new();
    for value in values {
        if let Value::Number(n) = value {
            if let Some(n) = n.as_u64() {
                numbers.push(n as u32);
            }
        }
    }

    Ok(numbers)
}

fn deserialize_hashmap<'de, D>(deserializer: D) -> Result<HashMap<usize, Option<u32>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let values: Vec<Value> = Vec::deserialize(deserializer)?;
    let iter = values.iter();

    let mut numbers: HashMap<usize, Option<u32>> = HashMap::new();
    for (i, value) in iter.enumerate() {
        if let Value::Number(n) = value {
            if let Some(n) = n.as_u64() {
                numbers.insert(i, Some(n as u32));
                continue;
            }
        } else if let Value::String(n) = value {
            if let Ok(n) = n.parse::<u32>() {
                numbers.insert(i, Some(n));
                continue;
            }
        }

        numbers.insert(i, None);
    }

    Ok(numbers)
}

fn deserialize_drivers<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let values = String::deserialize(deserializer)?;

    let mut drivers = Vec::new();

    for driver in values.split(", ") {
        drivers.push(driver.trim().to_string());
    }

    Ok(drivers)
}
