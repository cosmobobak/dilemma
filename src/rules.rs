use rand::Rng;

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
    rounds: usize,
    steadiness_millis: u32,
    cc_payoff: f64,
    cd_payoff: f64,
    dc_payoff: f64,
    dd_payoff: f64,
}

pub struct Results {
    player_one_choice: Choice,
    player_two_choice: Choice,
    player_one_payoff: f64,
    player_two_payoff: f64,
}

pub trait Player {
    fn choose(&mut self, your_history: &[Choice], their_history: &[Choice]) -> Choice;
    fn strategy(&self) -> String;
}

fn run_encounter(
    parameters: &GameParameters,
    p1_history: &[Choice],
    p2_history: &[Choice],
    player_one: &mut dyn Player,
    player_two: &mut dyn Player,
) -> Results {
    let player_one_choice = player_one.choose(p1_history, p2_history);
    let player_two_choice = player_two.choose(p2_history, p1_history);
    let mut rng = rand::thread_rng();
    let r_one = rng.gen_range(1..=1000);
    let r_two = rng.gen_range(1..=1000);
    let player_one_choice = if r_one > parameters.steadiness_millis {
        player_one_choice.flip()
    } else {
        player_one_choice
    };
    let player_two_choice = if r_two > parameters.steadiness_millis {
        player_two_choice.flip()
    } else {
        player_two_choice
    };

    let player_one_payoff = match (player_one_choice, player_two_choice) {
        (Choice::Cooperate, Choice::Cooperate) => parameters.cc_payoff,
        (Choice::Cooperate, Choice::Defect) => parameters.cd_payoff,
        (Choice::Defect, Choice::Cooperate) => parameters.dc_payoff,
        (Choice::Defect, Choice::Defect) => parameters.dd_payoff,
    };

    let player_two_payoff = match (player_two_choice, player_one_choice) {
        (Choice::Cooperate, Choice::Cooperate) => parameters.cc_payoff,
        (Choice::Cooperate, Choice::Defect) => parameters.cd_payoff,
        (Choice::Defect, Choice::Cooperate) => parameters.dc_payoff,
        (Choice::Defect, Choice::Defect) => parameters.dd_payoff,
    };

    Results {
        player_one_choice,
        player_two_choice,
        player_one_payoff,
        player_two_payoff,
    }
}

pub fn run_match(
    parameters: &GameParameters,
    player_one: &mut dyn Player,
    player_two: &mut dyn Player,
) -> (f64, f64) {
    let mut p1_history = Vec::new();
    let mut p2_history = Vec::new();
    let mut utilities = (0.0, 0.0);

    for _ in 0..parameters.rounds {
        let result = run_encounter(parameters, &p1_history, &p2_history, player_one, player_two);
        p1_history.push(result.player_one_choice);
        p2_history.push(result.player_two_choice);
        utilities.0 += result.player_one_payoff;
        utilities.1 += result.player_two_payoff;
    }

    utilities
}

impl Default for GameParameters {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let rounds = rng.gen_range(180..=220);
        Self {
            rounds,
            steadiness_millis: 1000,
            cc_payoff: 3.0,
            cd_payoff: 0.0,
            dc_payoff: 5.0,
            dd_payoff: 1.0,
        }
    }
}
