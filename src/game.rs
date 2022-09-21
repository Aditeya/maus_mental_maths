use crate::dice::Dice;

pub struct Game {
    pub die: Dice,
    pub a: u32,
    pub b: u32,
}

impl Default for Game {
    fn default() -> Self {
        Game::new()
    }
}

impl Game {
    pub fn new() -> Self {
        let die = Dice::new(1, 10);
        let a = die.roll();
        let b = die.roll();

        Self {
            die,
            a,
            b,
        }
    }

    pub fn next_question(&mut self) {
        self.a = self.die.roll();
        self.b = self.die.roll();
    }

    pub fn answer(&self) -> u32 {
        self.a * self.b
    }
}
