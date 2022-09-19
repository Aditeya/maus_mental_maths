use rand::Rng;

pub struct Dice {
    start: u32,
    end: u32,
}

impl Dice {
    pub fn new(start: u32, end: u32) -> Self {
        Self {
            start,
            end,
        }
    }

    pub fn roll(&self) -> u32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(self.start..=self.end)
    }
}
