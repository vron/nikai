use crate::rai;
use rai::Action::*;

pub struct Ai();

impl rai::Ai for Ai {
    fn run<'a>(&self, e: Box<dyn rai::AiEnv + 'a>) -> rai::Action {
        if e.no_resources(0.0, 0.0) > 0.0 {
            return Create;
        }

        if e.no_enemies(0.0, 0.0) > 0.0 && e.no_enemies(0.0, 0.0) < e.no_friends(0.0, 0.0) {
            return Nothing;
        }

        let r: u8 = 2;
        return match r % 8 {
            0 => MoveN,
            1 => MoveNE,
            2 => MoveE,
            3 => MoveSE,
            4 => MoveS,
            5 => MoveSW,
            6 => MoveW,
            7 => MoveNW,
            _ => Nothing,
        };
    }
}
