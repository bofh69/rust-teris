extern crate pancurses;

use crate::model::Game;
use crate::model::PieceType;
use pancurses::Window;

const OFFSET_X: u8 = 2;
const OFFSET_Y: u8 = 2;

fn init_colours()
{
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

    // Board decoration colour
    pancurses::init_pair(8, pancurses::COLOR_YELLOW, pancurses::COLOR_BLACK);
}

fn draw_board_decoration(win: &Window, width: u8, height: u8)
{
    win.color_set(8);
    for y in (OFFSET_Y) .. (OFFSET_Y + height) {
        win.mv(i32::from(y), i32::from(OFFSET_X-1));
        win.addch('|');
        win.mv(i32::from(y), i32::from(OFFSET_X + width*2));
        win.addch('|');
    }
    for x in (OFFSET_X) .. (OFFSET_X + width*2) {
        win.mv(i32::from(OFFSET_Y - 1), i32::from(x));
        win.addch('-');
        win.mv(i32::from(OFFSET_Y + height), i32::from(x));
        win.addch('-');
    }
    win.mv(i32::from(OFFSET_Y-1), i32::from(OFFSET_X-1));
    win.addch('+');
    win.mv(i32::from(OFFSET_Y-1), i32::from(OFFSET_X+width*2));
    win.addch('+');
    win.mv(i32::from(OFFSET_Y+height), i32::from(OFFSET_X-1));
    win.addch('+');
    win.mv(i32::from(OFFSET_Y+height), i32::from(OFFSET_X+width*2));
    win.addch('+');
}

/// Inits the curses.
pub fn init(width:u8, height:u8) -> Window {
    let win = pancurses::initscr();
    win.nodelay(true);
    win.scrollok(false);
    pancurses::cbreak();
    pancurses::noecho();

    init_colours();
    draw_board_decoration(&win, width, height);

    win
}

// Move to different place.
pub fn draw_in_win(g: &Game, win: &Window) {
    fn set_color(win: &Window, c: &PieceType) {
        let cp = match *c {
            PieceType::None => 0,
            PieceType::I => 1,
            PieceType::J => 2,
            PieceType::L => 3,
            PieceType::O => 4,
            PieceType::S => 5,
            PieceType::T => 6,
            PieceType::Z => 7,
        };
        win.color_set(cp);
    }

    let width = g.board.width() as usize;
    for y in 0..g.board.height() {
        win.mv(i32::from(y + OFFSET_Y), i32::from(OFFSET_X));
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
