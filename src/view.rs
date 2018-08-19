extern crate pancurses;

use pancurses::Window;
use model::Game;
use model::PieceType;

/// Inits the curses.
pub fn init() -> Window {
    let win = pancurses::initscr();
    win.nodelay(true);
    win.scrollok(false);
    pancurses::cbreak();
    pancurses::noecho();

    pancurses::start_color();

    pancurses::init_pair(0, pancurses::COLOR_WHITE, pancurses::COLOR_BLACK);
    // I CYAN
    pancurses::init_pair(1, pancurses::COLOR_CYAN, pancurses::COLOR_CYAN);
    // J BLUE
    pancurses::init_pair(2, pancurses::COLOR_BLUE, pancurses::COLOR_BLUE);
    // L ORANGE
    pancurses::init_pair(3, pancurses::COLOR_WHITE, pancurses::COLOR_WHITE);
    // O YELLOW
    pancurses::init_pair(4, pancurses::COLOR_YELLOW, pancurses::COLOR_YELLOW);
    // S LIME
    pancurses::init_pair(5, pancurses::COLOR_GREEN, pancurses::COLOR_GREEN);
    // T PURPLE
    pancurses::init_pair(6, pancurses::COLOR_MAGENTA, pancurses::COLOR_MAGENTA);
    // Z RED
    pancurses::init_pair(7, pancurses::COLOR_RED, pancurses::COLOR_RED);

    // FIXME: Draw the game-board here?

    win
}

// Move to different place.
pub fn draw_in_win(g: &Game, win: &Window) {

    fn set_color(win: &Window, c: &PieceType) {
        let cp = match c {
            &PieceType::None => 0,
            &PieceType::I => 1,
            &PieceType::J => 2,
            &PieceType::L => 3,
            &PieceType::O => 4,
            &PieceType::S => 5,
            &PieceType::T => 6,
            &PieceType::Z => 7,
        };
        win.color_set(cp);
    }

    let width = g.board.width() as usize;
    for y in 0..g.board.height() {
        win.mv(y as i32, 0);
        for x in 0..width {
            let o = match g.board.map[x + y as usize * width].clone() {
                PieceType::None => {
                    set_color(win, &PieceType::None);
                    '.'
                }
                c => {
                    set_color(win, &c);
                    'X'
                }
            };
            win.addch(o);
            win.addch(o);
        }
    }
}

/// Ends the GUI.
pub fn end() {
    pancurses::endwin();
}

#[cfg(test)]
mod tests {}
