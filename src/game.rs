use crate::dice::Dice;
use crate::op::OP;

pub struct Game {
    pub die: Dice,
    pub a: u32,
    pub b: u32,
    pub op: OP,
    rand_op: bool,
}

impl Default for Game {
    fn default() -> Self {
        Game::new(OP::Mul, false)
    }
}

impl Game {
    pub fn new(op: OP, rand_op: bool) -> Self {
        let die = Dice::new(1, 10);
        let a = die.roll();
        let b = die.roll();

        Self {
            die,
            a,
            b,
            op,
            rand_op,
        }
    }

    pub fn next_question(&mut self) {
        if self.rand_op {
            self.op = rand::random();
        }

        self.a = self.die.roll();

        loop {
            self.b = self.die.roll();
            if self.b <= self.a { break; }
        }
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
