#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use rules::{Player, run_match, GameParameters};

mod rules;
mod premades;
mod external;

const INTERNAL_PREFIX_CHAR: char = '~';

fn parse_program_id(program_id: &str) -> Result<Box<dyn Player>, String> {
    if program_id.starts_with(INTERNAL_PREFIX_CHAR) {
        match program_id.trim_start_matches(INTERNAL_PREFIX_CHAR).replace('-', "_").as_str() {
            "always_defect" => return Ok(Box::new(premades::AlwaysDefect)),
            "always_cooperate" => return Ok(Box::new(premades::AlwaysCooperate)),
            "grim_trigger" => return Ok(Box::<premades::GrimTrigger>::default()),
            "tit_for_tat" => return Ok(Box::new(premades::TitForTat)),
            "forgiving_tit_for_tat" => return Ok(Box::<premades::ForgivingTitForTat>::default()),
            "tit_for_two_tats" => return Ok(Box::new(premades::TitForTwoTats)),
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
        program_id.into(),
        engine,
        stdout,
        stdin,
    )))
}

fn main() {
    let mut args = std::env::args().collect::<Vec<_>>();
    match args.len() {
        0 => unreachable!("How did you even do this?"),
        1 | 2 => return eprintln!("Usage: {} <player_one> <player_two>", args[0]),
        _ => (),
    }

    let program_one = std::mem::take(&mut args[1]);
    let program_two = std::mem::take(&mut args[2]);

    let mut player_one = parse_program_id(&program_one).unwrap();
    let mut player_two = parse_program_id(&program_two).unwrap();

    let res = run_match(&GameParameters::default(), &mut *player_one, &mut *player_two);

    println!("match result: \n{}: {} utility\n{}: {} utility", player_one.strategy(), res.0, player_two.strategy(), res.1);
}
