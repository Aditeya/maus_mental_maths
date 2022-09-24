use rand::distributions::{Distribution, Standard};
use rand::Rng;

pub enum OP {
    Add,
    Sub,
    Mul,
    Div,
}

impl Distribution<OP> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> OP {
        match rng.gen_range(0..=3) {
            0 => OP::Add,
            1 => OP::Sub,
            2 => OP::Mul,
            _ => OP::Div,
        }
    }
}
