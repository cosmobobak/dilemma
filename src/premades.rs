use crate::rules::{Player, Choice};

/// Defects no matter what.
pub struct AlwaysDefect;

impl Player for AlwaysDefect {
    fn choose(&mut self, _: &[Choice], _: &[Choice]) -> Choice {
        Choice::Defect
    }

    fn strategy(&self) -> String {
        "always_defect".into()
    }
}

/// Cooperates no matter what.
pub struct AlwaysCooperate;

impl Player for AlwaysCooperate {
    fn choose(&mut self, _: &[Choice], _: &[Choice]) -> Choice {
        Choice::Cooperate
    }

    fn strategy(&self) -> String {
        "always_cooperate".into()
    }
}

/// Cooperates until you defect, then defects forever.
#[derive(Default)]
pub struct GrimTrigger {
    triggered: bool,
}

impl Player for GrimTrigger {
    fn choose(&mut self, _: &[Choice], their_history: &[Choice]) -> Choice {
        if self.triggered {
            Choice::Defect
        } else {
            let Some(most_recent_opponent_action) = their_history.last() else {
                return Choice::Cooperate;
            };
            if *most_recent_opponent_action == Choice::Defect {
                self.triggered = true;
                Choice::Defect
            } else {
                Choice::Cooperate
            }
        }
    }

    fn strategy(&self) -> String {
        "grim_trigger".into()
    }
}

/// Does whatever you just did to it. Cooperates on the first turn.
pub struct TitForTat;

impl Player for TitForTat {
    fn choose(&mut self, _: &[Choice], their_history: &[Choice]) -> Choice {
        their_history.last().copied().unwrap_or(Choice::Cooperate)
    }

    fn strategy(&self) -> String {
        "tit_for_tat".into()
    }
}

/// Cooperates on the first turn, then does whatever you just did to it, but will randomly forgive you.
pub struct ForgivingTitForTat {
    forgivingness_millis: u32,
}

impl Default for ForgivingTitForTat {
    fn default() -> Self {
        Self { forgivingness_millis: 50 }
    }
}

impl Player for ForgivingTitForTat {
    fn choose(&mut self, _: &[Choice], their_history: &[Choice]) -> Choice {
        let Some(most_recent_opponent_action) = their_history.last() else {
            return Choice::Cooperate;
        };

        let mut rng = rand::thread_rng();
        let r = rand::Rng::gen_range(&mut rng, 1..=1000);

        // if r beats the forgivingness threshold, then we cooperate unconditionally
        // eg if forgivingness_millis is 1000, then we always cooperate
        // normal values for forgivingness are 5%, so forgivingness_millis would be 50
        if r > self.forgivingness_millis {
            *most_recent_opponent_action
        } else {
            Choice::Cooperate
        }
    }

    fn strategy(&self) -> String {
        "forgiving_tit_for_tat".into()
    }
}

/// Cooperates on the first turn, and only defects if you defect against it twice in a row.
pub struct TitForTwoTats;

impl Player for TitForTwoTats {
    fn choose(&mut self, _: &[Choice], their_history: &[Choice]) -> Choice {
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

    fn strategy(&self) -> String {
        "tit_for_two_tats".into()
    }
}