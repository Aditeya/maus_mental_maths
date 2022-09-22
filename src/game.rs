use crate::dice::Dice;
use crate::op::OP;

pub struct Game {
    pub die: Dice,
    pub a: u32,
    pub b: u32,
    pub op: OP,
}

impl Default for Game {
    fn default() -> Self {
        Game::new(OP::Mul)
    }
}

impl Game {
    pub fn new(op: OP) -> Self {
        let die = Dice::new(1, 10);
        let a = die.roll();
        let b = die.roll();

        Self {
            die,
            a,
            b,
            op,
        }
    }

    pub fn next_question(&mut self) {
        self.a = self.die.roll();
        self.b = self.die.roll();
    }

    pub fn answer(&self) -> u32 {
        match &self.op {
            OP::Add => self.a + self.b,
            OP::Sub => self.a - self.b,
            OP::Mul => self.a * self.b,
            OP::Div => self.a / self.b,
        }
    }
}
