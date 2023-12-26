use crate::rules::{Choice, Player};

/// Defects no matter what.
pub struct AlwaysDefect;

impl Player for AlwaysDefect {
    fn choose(&self, _: &[Choice], _: &[Choice], _: &mut fastrand::Rng) -> Choice {
        Choice::Defect
    }
}

/// Cooperates no matter what.
pub struct AlwaysCooperate;

impl Player for AlwaysCooperate {
    fn choose(&self, _: &[Choice], _: &[Choice], _: &mut fastrand::Rng) -> Choice {
        Choice::Cooperate
    }
}

/// Cooperates until you defect, then defects forever.
#[derive(Default)]
pub struct GrimTrigger;

impl Player for GrimTrigger {
    fn choose(&self, _: &[Choice], their_history: &[Choice], _: &mut fastrand::Rng) -> Choice {
        if their_history.contains(&Choice::Defect) {
            Choice::Defect
        } else {
            Choice::Cooperate
        }
    }
}

/// Does whatever you just did to it. Cooperates on the first turn.
pub struct TitForTat;

impl Player for TitForTat {
    fn choose(&self, _: &[Choice], their_history: &[Choice], _: &mut fastrand::Rng) -> Choice {
        their_history.last().copied().unwrap_or(Choice::Cooperate)
    }
}

/// Cooperates on the first turn, then does whatever you just did to it, but will randomly forgive you.
pub struct ForgivingTitForTat {
    forgivingness_millis: u32,
}

impl Default for ForgivingTitForTat {
    fn default() -> Self {
        Self {
            forgivingness_millis: 50,
        }
    }
}

impl Player for ForgivingTitForTat {
    fn choose(
        &self,
        _: &[Choice],
        their_history: &[Choice],
        rng: &mut fastrand::Rng,
    ) -> Choice {
        let Some(most_recent_opponent_action) = their_history.last() else {
            return Choice::Cooperate;
        };

        let r = rng.u32(1..=1000);

        // if r beats the forgivingness threshold, then we cooperate unconditionally
        // eg if forgivingness_millis is 1000, then we always cooperate
        // normal values for forgivingness are 5%, so forgivingness_millis would be 50
        if r > self.forgivingness_millis {
            *most_recent_opponent_action
        } else {
            Choice::Cooperate
        }
    }
}

/// Cooperates on the first turn, and only defects if you defect against it twice in a row.
pub struct TitForTwoTats;

impl Player for TitForTwoTats {
    fn choose(&self, _: &[Choice], their_history: &[Choice], _: &mut fastrand::Rng) -> Choice {
        if their_history.len() < 2 {
            return Choice::Cooperate;
        }

        let one_ago = their_history[their_history.len() - 1];
        let two_ago = their_history[their_history.len() - 2];

        match (one_ago, two_ago) {
            (Choice::Defect, Choice::Defect) => Choice::Defect,
            _ => Choice::Cooperate,
        }
    }
}

/// Randomly cooperates or defects.
pub struct Random;

impl Player for Random {
    fn choose(&self, _: &[Choice], _: &[Choice], rng: &mut fastrand::Rng) -> Choice {
        if rng.bool() {
            Choice::Cooperate
        } else {
            Choice::Defect
        }
    }
}

/// Guesses what you're going to do, then matches it.
pub struct SimpleGuesser;

impl Player for SimpleGuesser {
    #[allow(clippy::cast_precision_loss)]
    fn choose(
        &self,
        our_history: &[Choice],
        their_history: &[Choice],
        rng: &mut fastrand::Rng,
    ) -> Choice {
        if our_history.len() < 2 {
            // acts like tit-for-tat for the first two rounds
            return their_history.last().copied().unwrap_or(Choice::Cooperate);
        }
        // then, we start conditionalising on what the opponent has done in the past:
        let mut circumstances = [0.0; 4];
        for ((our_prior, their_prior), outcome) in our_history
            .iter()
            .zip(their_history)
            .zip(their_history.iter().skip(1))
        {
            let index = *our_prior as usize * 2 + *their_prior as usize;
            circumstances[index] += f64::from(*outcome as i16);
        }
        // normalise:
        for c in &mut circumstances {
            *c /= (their_history.len() - 1) as f64;
        }
        // what circumstance are we in right now?
        let index = our_history[our_history.len() - 1] as usize * 2
            + their_history[their_history.len() - 1] as usize;
        let p_defect = circumstances[index];

        if rng.f64() < p_defect {
            Choice::Defect
        } else {
            Choice::Cooperate
        }
    }
}
