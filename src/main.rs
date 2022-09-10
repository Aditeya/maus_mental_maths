use read_input::prelude::*;
use rand::distributions::{Distribution, Uniform};
use colored::Colorize;

fn main() {
    println!("Welcome to Maus Mental Maths Trainer\nLet's Begin!\n");

    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..=10);

    let mut correct = 0;
    let mut wrong = 0;

    loop {
        let a: u32 = die.sample(&mut rng);
        let b: u32 = die.sample(&mut rng);
        print!("{} × {} = ", a, b);
        let input = input::<String>().get();

        if input == "q" || input == "Q" {
            println!("\nExiting...\n");
            println!("{} : {}", "CORRECT".green(), correct);
            println!("{}   : {}", "WRONG".red(), wrong);
            println!("TOTAL   : {}", correct + wrong);
            println!("\nThanks for playing!");
            break;
        }

        let mut ans = 0;
        if let Ok(n) = input.parse::<u32>() {
            ans = n;
        }

        let ab = a*b;
        if ans == ab {
            println!("{}\n", "CORRECT".green());
            correct += 1;
        } else {
            println!("{} {}\n", "WRONG:".red(), ab);
            wrong += 1;
        }
    }
}
