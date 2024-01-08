use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Choice {
    Cooperate,
    Defect,
}

impl Choice {
    pub const fn flip(self) -> Self {
        match self {
            Self::Cooperate => Self::Defect,
            Self::Defect => Self::Cooperate,
        }
    }
}

pub struct GameParameters {
    mean_rounds: usize,
    steadiness_millis: u32,
    cc_payoff: f64,
    cd_payoff: f64,
    dc_payoff: f64,
    dd_payoff: f64,
}

pub struct Results {
    one_choice: Choice,
    two_choice: Choice,
    one_payoff: f64,
    two_payoff: f64,
}

pub trait Player {
    fn choose(
        &self,
        your_history: &[Choice],
        their_history: &[Choice],
        rng: &mut fastrand::Rng,
    ) -> Choice;
}

fn run_encounter(
    parameters: &GameParameters,
    p1_history: &[Choice],
    p2_history: &[Choice],
    player_one: &dyn Player,
    player_two: &dyn Player,
    rng: &mut fastrand::Rng,
) -> Results {
    let one_choice = player_one.choose(p1_history, p2_history, rng);
    let two_choice = player_two.choose(p2_history, p1_history, rng);
    let r_one = rng.u32(1..=1000);
    let r_two = rng.u32(1..=1000);
    let one_choice = if r_one > parameters.steadiness_millis {
        one_choice.flip()
    } else {
        one_choice
    };
    let two_choice = if r_two > parameters.steadiness_millis {
        two_choice.flip()
    } else {
        two_choice
    };

    let one_payoff = match (one_choice, two_choice) {
        (Choice::Cooperate, Choice::Cooperate) => parameters.cc_payoff,
        (Choice::Cooperate, Choice::Defect) => parameters.cd_payoff,
        (Choice::Defect, Choice::Cooperate) => parameters.dc_payoff,
        (Choice::Defect, Choice::Defect) => parameters.dd_payoff,
    };

    let two_payoff = match (two_choice, one_choice) {
        (Choice::Cooperate, Choice::Cooperate) => parameters.cc_payoff,
        (Choice::Cooperate, Choice::Defect) => parameters.cd_payoff,
        (Choice::Defect, Choice::Cooperate) => parameters.dc_payoff,
        (Choice::Defect, Choice::Defect) => parameters.dd_payoff,
    };

    Results {
        one_choice,
        two_choice,
        one_payoff,
        two_payoff,
    }
}

#[allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
pub fn one_on_one_match(
    parameters: &GameParameters,
    player_one: &dyn Player,
    player_two: &dyn Player,
    rng: &mut fastrand::Rng,
) -> (f64, f64) {
    let mut p1_history = Vec::new();
    let mut p2_history = Vec::new();
    let mut utilities = (0.0, 0.0);

    // exponential distribution with mean `mean_rounds`:
    let lambda = 1.0 / parameters.mean_rounds as f64;
    let rounds = (-f64::ln(rng.f64()) / lambda).round() as usize;
    for _ in 0..rounds {
        let result = run_encounter(
            parameters,
            &p1_history,
            &p2_history,
            player_one,
            player_two,
            rng,
        );
        p1_history.push(result.one_choice);
        p2_history.push(result.two_choice);
        utilities.0 += result.one_payoff;
        utilities.1 += result.two_payoff;
    }

    utilities
}

/// Runs round-robin (everyone plays everyone) matches,
/// then removes the worst-performing player, replacing it with a random selection from the rest.
pub fn tournament(
    generations: usize,
    initial_population: &[String],
    player_map: &HashMap<String, Box<dyn Player>>,
) -> HashMap<String, f64> {
    let mut accrued_utilities = HashMap::new();
    let mut population = initial_population.to_vec();
    let mut utilities = vec![0.0; initial_population.len()];
    let mut type_counts: HashMap<String, u64> = HashMap::new();
    let mut rng = fastrand::Rng::new();
    let max_name_len = population.iter().map(String::len).max().unwrap() + 1;
    for g in 1..=generations {
        println!("Generation {g}");
        // simulation:
        for (p1dex, p1) in population.iter().enumerate() {
            for (p2dex, p2) in population.iter().enumerate().skip(p1dex + 1) {
                let player_one = player_map.get(p1).unwrap().as_ref();
                let player_two = player_map.get(p2).unwrap().as_ref();
                let (p1_utility, p2_utility) =
                    one_on_one_match(&GameParameters::default(), player_one, player_two, &mut rng);
                utilities[p1dex] += p1_utility;
                utilities[p2dex] += p2_utility;

                *accrued_utilities.entry(p1.clone()).or_insert(0.0) += p1_utility;
                *accrued_utilities.entry(p2.clone()).or_insert(0.0) += p2_utility;
            }
        }
        // selection:
        let mut worst_performer = 0;
        let mut worst_utility = utilities[0];
        for (i, u) in utilities.iter().copied().enumerate().skip(1) {
            if u < worst_utility {
                worst_utility = u;
                worst_performer = i;
            }
        }
        let random_selection = rng.usize(0..population.len());
        population[worst_performer] = population[random_selection].clone();
        // reporting:
        for i in &population {
            *type_counts.entry(i.clone()).or_insert(0) += 1;
        }
        let mut counts_vec = type_counts.drain().collect::<Vec<_>>();
        counts_vec.sort_unstable_by(|a, b| b.1.cmp(&a.1));
        for (strategy, count) in counts_vec {
            let total_u = population
                .iter()
                .zip(&utilities)
                .filter(|(s, _)| **s == strategy)
                .map(|(_, u)| *u)
                .sum::<f64>();
            #[allow(clippy::cast_precision_loss)]
            let mean_utility = total_u / count as f64;
            println!("  {strategy:<max_name_len$}: {count:<5} (mean utility {mean_utility:.0})");
        }
        // reset:
        utilities.fill(0.0);
    }

    accrued_utilities
}

impl Default for GameParameters {
    fn default() -> Self {
        Self {
            mean_rounds: 150,
            steadiness_millis: 1000,
            cc_payoff: 3.0,
            cd_payoff: 0.0,
            dc_payoff: 5.0,
            dd_payoff: 1.0,
        }
    }
}
