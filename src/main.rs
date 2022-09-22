use read_input::prelude::*;
use colored::Colorize;

use maus_mental_maths::game::Game;
use maus_mental_maths::op::OP;

fn main() {
    println!("Welcome to Maus Mental Maths Trainer\nLet's Begin!\n");

    let mut correct = 0;
    let mut wrong = 0;
    let mut game = Game::new(OP::Mul);

    loop {
        print!("{} × {} = ", game.a, game.b);
        let input = input::<String>().get();

       if input == "q" || input == "Q" {
            game_end(correct, wrong);
            break;
        }

        let mut ans = 0;
        if let Ok(n) = input.parse::<u32>() {
            ans = n;
        }

        let ab = game.answer();
        if ans == ab {
            println!("{}\n", "CORRECT".green());
            correct += 1;
        } else {
            println!("{} {}\n", "WRONG:".red(), ab);
            wrong += 1;
        }

        game.next_question();
    }
}

fn game_end(correct: u32, wrong: u32) {
     println!("\nExiting...\n\n{} : {}", "CORRECT".green(), correct);
     println!("{}   : {}", "WRONG".red(), wrong);
     println!("TOTAL   : {}\n\nThanks for playing!", correct + wrong);
}
