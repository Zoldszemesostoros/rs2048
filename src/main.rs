mod arrowwait;
use arrowwait::arrow_wait;

mod game;
use game::Table;

use ncurses::*;
use std::time::Duration;

/// Entry point of the program.
fn main() {
    let mut tbl = Table::new();
    initscr();
    noecho();
    tbl.add_tile().expect("Cannot add new tile. :(");
    tbl.add_tile().expect("Cannot add new tile. :(");
    loop {
        addstr(&format!("{}", tbl));
        let arrow_code = arrow_wait(Duration::from_millis(100));
        clear();
        let tbl_old = tbl.clone();
        match arrow_code {
            0 => tbl.swipe_up(),
            1 => tbl.swipe_down(),
            2 => tbl.swipe_right(),
            3 => tbl.swipe_left(),
            _ => panic!("Oh no, arrow detection failed :("),
        }
        if tbl != tbl_old {
            tbl.add_tile().expect("Cannot add new tile. :(");
        } else {
            if tbl.is_game_over() {
                println!("Game over.");
                std::process::exit(0);
            }
        }
        refresh();
    }
}
