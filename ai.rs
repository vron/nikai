use crate::rai;
use rai::Action::*;

pub struct Ai();

impl rai::Ai for Ai {
    fn run<'a>(&self, e: Box<dyn rai::AiEnv + 'a>) -> rai::Action {
        let r = self.rnd(e);
        let r: u8 = (r*9.0) as u8;
        return match r % 9 {
            0 => MoveN,
            1 => MoveNE,
            2 => MoveE,
            3 => MoveSE,
            4 => MoveS,
            5 => MoveSW,
            6 => MoveW,
            7 => MoveNW,
            8 => Create,
            _ => Nothing,
        };
    }
}

impl Ai {
    fn rnd<'a>(&self, mut e: Box<dyn rai::AiEnv + 'a>) -> f32 {
        let s1 = e.get_memory(0.0);
        let s2 = s1 + 11.34;
        let s3 = s2.rem_euclid(1.0);
        //eprintln!("{} {} {}", s1, s2, s3);
        e.set_memory(0.0, s3);
        s3
    }
}
