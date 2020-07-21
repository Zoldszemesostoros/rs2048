use ncurses::*;
use std::thread::sleep;
use std::time::Duration;
/// Waits for an arrow key
/// Returns 0-3.
pub fn arrow_wait(sleep_dur: Duration) -> i32 {

    // Arrow keys are represented with escape sequences:
    // 0x1b (esc), 0x5b (opening square bracket), and 0x41-0x44, depending on the arrow.
    let mut after_esc = false;
    let mut after_bracket = false;
    let result = loop {

        let ch = getch();
        if after_bracket && ch >= 0x41 && ch <= 0x44 {
            break ch - 0x41;
        }
        if ch == 0x5b && after_esc {
            after_bracket = true;
        } else {
            after_bracket = false;
        }
        if ch == 0x1b {
            after_esc = true;
        } else {
            after_esc = false;
        }
        
        refresh();
        sleep(sleep_dur);
    };
    result
}