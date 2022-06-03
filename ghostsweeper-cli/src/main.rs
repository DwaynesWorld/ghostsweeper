use ghostsweeper_engine::ghostsweeper::Ghostsweeper;
use std::io;
use std::option::Option;

enum Action {
    Open(usize, usize),
    Flag(usize, usize),
    Quit,
}

fn parse_action(action: &str) -> Option<Action> {
    let action = action.to_lowercase();

    if action.starts_with("open") || action.starts_with("flag") {
        let mut iter = action.split_ascii_whitespace();
        iter.next();
        let x: i32 = iter.next().unwrap().parse().unwrap();
        let x = (x - 1) as usize;

        let y: i32 = iter.next().unwrap().parse().unwrap();
        let y = (y - 1) as usize;
        if action.starts_with("flag") {
            return Some(Action::Flag(x, y));
        }

        return Some(Action::Open(x, y));
    }

    if action.starts_with("quit") {
        return Some(Action::Quit);
    }

    None
}

fn start_game_loop() {
    let mut game = Ghostsweeper::new(10, 5, 2);

    loop {
        println!("{}", game);
        println!("");
        println!("Enter move: ");

        let mut action = String::new();

        io::stdin()
            .read_line(&mut action)
            .expect("Failed to read line");

        let action = parse_action(&action.trim()).unwrap();
        match action {
            Action::Open(x, y) => {
                game.open((x, y));
            }
            Action::Flag(x, y) => {
                game.toggle_flag((x, y));
            }
            Action::Quit => {
                println!("quitting!");
            }
        }

        game.check_state();
        if game.is_over {
            println!("{}", game);
            if game.is_winner {
                println!("You won!");
            } else {
                println!("You lost!");
            }

            return;
        }
    }
}

fn main() {
    println!("Ghostsweeper!");
    println!("Do you want to play. [Y/n]");

    let mut play = String::new();

    io::stdin()
        .read_line(&mut play)
        .expect("Failed to read line");

    if play.trim().to_lowercase().eq("y") {
        start_game_loop();
    }
}
