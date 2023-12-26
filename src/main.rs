#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::collections::HashMap;

use rules::{Player, tournament};

mod external;
mod premades;
mod rules;

const INTERNAL_PREFIX_CHAR: char = '~';

fn parse_program_id(program_id: &str) -> Result<Box<dyn Player>, String> {
    if program_id.starts_with(INTERNAL_PREFIX_CHAR) {
        match program_id
            .trim_start_matches(INTERNAL_PREFIX_CHAR)
            .replace('-', "_")
            .as_str()
        {
            "always_defect"         => return Ok(Box::new(premades::AlwaysDefect)),
            "always_cooperate"      => return Ok(Box::new(premades::AlwaysCooperate)),
            "grim_trigger"          => return Ok(Box::<premades::GrimTrigger>::default()),
            "tit_for_tat"           => return Ok(Box::new(premades::TitForTat)),
            "forgiving_tit_for_tat" => return Ok(Box::<premades::ForgivingTitForTat>::default()),
            "tit_for_two_tats"      => return Ok(Box::new(premades::TitForTwoTats)),
            "random"                => return Ok(Box::new(premades::Random)),
            "simple_guesser"        => return Ok(Box::new(premades::SimpleGuesser)),
            other => return Err(format!("{other} isn't an inbuilt strategy!")),
        }
    }

    // assume we have a path to a program binary:
    let mut engine = std::process::Command::new(program_id)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to start engine");

    let stdin = engine.stdin.take().unwrap();
    let stdout = engine.stdout.take().unwrap();

    Ok(Box::new(external::ExePlayer::new(
        engine,
        stdout,
        stdin,
    )))
}

fn main() {
    let mut population = vec![
        "~always_defect".to_string(),
        "~always_cooperate".to_string(),
        "~grim_trigger".to_string(),
        "~tit_for_tat".to_string(),
        "~forgiving_tit_for_tat".to_string(),
        "~tit_for_two_tats".to_string(),
        "~random".to_string(),
        "~simple_guesser".to_string(),
    ];

    population.extend(std::env::args().skip(1));

    let player_map: HashMap<String, Box<dyn Player>> = population
        .iter()
        .map(|p| (p.to_owned(), parse_program_id(p).unwrap()))
        .collect();

    // grow each strategy to 50 individuals:
    let population = population
        .into_iter()
        .flat_map(|s| vec![s; 15])
        .collect::<Vec<_>>();

    // run the tournament:
    tournament(1_000, &population, &player_map);
}
