use std::{collections::HashMap, fs};

use clap::{builder::Str, Parser};

use crate::output::{Round, Standings};

mod input;
mod output;

#[derive(serde::Deserialize, Debug)]
struct Config {
    rf2la_url: String,
    championship_id: u32,
    target_file: String,
    car_class: String,
    car_numbers: HashMap<String, u32>,
}

fn main() {
    let config: Config = toml::from_str(&fs::read_to_string("config.toml").unwrap()).unwrap();

    let url = format!(
        "{}/championships/export_standings_json.json?cid={}",
        config.rf2la_url, config.championship_id
    );

    println!("{}", url);

    // let req: input::Outer = reqwest::blocking::get(url).unwrap().json().unwrap();

    let req: input::Outer = reqwest::blocking::get(url).unwrap().json().unwrap();

    let mut total_rounds = 0;
    let mut standings = Vec::new();
    let teams = req.class_overall.team_standings.teams;

    for driver in req.class_overall.driver_standings.drivers.values() {
        let driver_rounds = driver.points.iter().filter(|p| p.1.is_some()).count();
        if driver_rounds > total_rounds {
            total_rounds = driver_rounds;
        }
    }

    for (position, driver) in req.class_overall.driver_standings.drivers {
        let team_name = teams
            .iter()
            .find(|t| t.1.team_drivers.contains(&driver.name))
            .map(|t| t.1.team_name.clone())
            .unwrap_or("No team".to_string());

        let Some(car_number) = config.car_numbers.get(&driver.name.to_lowercase()).cloned() else {
            panic!("No car number found for driver {}", driver.name.to_lowercase());
        };

        let mut rounds = vec![];

        for i in 0..total_rounds {
            rounds.push(Round {
                points: driver.points.get(&i).map(|p| p.unwrap_or(0)).unwrap_or(0),
            });
        }

        standings.push(Standings {
            position,
            car_number,
            driver_name: driver.name,
            full_team_name: team_name,
            car_class: config.car_class.clone(),
            total_points: driver.points_sum,
            rounds,
        });
    }

    standings.sort_by(|a, b| a.position.cmp(&b.position));

    let output = output::Outer {
        total_rounds,
        standings,
    };

    fs::write(
        config.target_file,
        serde_json::to_string_pretty(&output).unwrap(),
    )
    .unwrap();
}
