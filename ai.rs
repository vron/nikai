use crate::rai;
use rai::Action::*;

pub struct Ai();

impl rai::Ai for Ai {
    fn run<'a>(&self, e: Box<dyn rai::AiEnv + 'a>) -> rai::Action {
        if e.no_resources(0.0, 0.0) > 0.0 {
            return Create;
        }

        if e.no_enemies(0.0, 0.0) > 0.0 {
            if e.no_friends(0.0, 0.0) >= e.no_enemies(0.0, 0.0){
                return Nothing;
            }
            return self.rnd_move(e);
        }

        let cases = vec![
            (0.0, 1.0, MoveN),
            (1.0, 1.0, MoveNE),
            (1.0, 0.0, MoveE),
            (1.0, -1.0, MoveSE),
            (0.0, -1.0, MoveS),
            (-1.0, -1.0, MoveSW),
            (-1.0, 0.0, MoveW),
            (-1.0, 1.0, MoveNW)
        ];
        // Harvest if resources on friendly ground
        for (dx, dy, a) in cases.into_iter() {
            if e.no_resources(dx,dy) > 0.0 && !(e.no_enemies(dx, dy) > 0.0) && !(e.no_friends(dx, dy) > 0.0) {
                return a;
            }
        }

        let cases = vec![
            (0.0, 1.0, MoveN),
            (1.0, 1.0, MoveNE),
            (1.0, 0.0, MoveE),
            (1.0, -1.0, MoveSE),
            (0.0, -1.0, MoveS),
            (-1.0, -1.0, MoveSW),
            (-1.0, 0.0, MoveW),
            (-1.0, 1.0, MoveNW)
        ];
        // Attract to fight if we are more
        for (dx, dy, a) in cases.into_iter() {
            if e.no_enemies(dx, dy) > 0.0 && e.no_friends(0.0, 0.0) > e.no_enemies(dx, dy){
                return a;
            }
        }

        return self.rnd_move(e)
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

    fn rnd2<'a>(&self, s1: f32) -> f32 {
        // let s1 = e.get_memory(0.0);
        let s2 = s1 + 16.78;
        let s3 = s2.rem_euclid(1.0);
        //eprintln!("{} {} {}", s1, s2, s3);
        // e.set_memory(0.0, s3);
        s3
      }

    fn rnd_move<'a>(&self, mut e: Box<dyn rai::AiEnv + 'a>) -> Action {
        // Favor exploration
      let mut mem = e.get_memory(0.0);
      let mut ra : u8 = (mem*9.0) as u8;
      mem = self.rnd2(mem);
      if mem > 0.5 {
        mem = self.rnd2(mem);
        ra = (mem*9.0) as u8;
      }
      e.set_memory(0.0, mem);

      return match ra % 9 {
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
