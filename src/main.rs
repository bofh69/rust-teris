// SPDX-FileCopyrightText: 2022 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: Apache-2.0

extern crate pancurses;
extern crate rand;

use pancurses::Window;

mod model;
mod view;

use self::model::Board;
use self::model::Game;
use self::model::PieceFactory;

fn left(g: &mut Game) {
    if g.pos.0 > 0 {
        g.clear();
        g.pos.0 -= 1;
        if !g.fits() {
            g.pos.0 += 1;
        }
        g.draw();
    }
}

fn right(g: &mut Game) {
    if g.pos.0 < g.board.width() as i8 - 1 {
        g.clear();
        g.pos.0 += 1;
        if !g.fits() {
            g.pos.0 -= 1;
        }
        g.draw();
    }
}

fn up(g: &mut Game) {
    g.clear();
    g.turn_piece();
    if !g.fits() {
        g.counter_turn_piece();
    }
    g.draw();
}

fn down(g: &mut Game) {
    g.clear();
    g.pos.1 += 1;
    if !g.fits() {
        g.pos.1 -= 1;
        g.draw();
        // FIXME:
        g.piece_stuck();
    }
    if !g.game_over {
        g.draw();
    }
}

fn fall(g: &mut Game) {
    g.clear();
    while g.fits() {
        g.pos.1 += 1;
    }
    g.pos.1 -= 1;
    g.draw();
    g.piece_stuck();
    g.draw();
}

fn game_loop(win: &Window, g: &mut Game) {
    use std::{thread, time};

    g.draw();
    let mut now = time::Instant::now();
    while !g.game_over {
        match win.getch() {
            Some(pancurses::Input::Character(c)) => match c {
                'q' => return,
                'h' => left(g),
                'l' => right(g),
                'k' => up(g),
                'j' => {
                    now = time::Instant::now();
                    down(g)
                }
                ' ' => {
                    now = time::Instant::now();
                    fall(g)
                }
                _ => (),
            },
            Some(_) => (),
            None => {
                thread::sleep(time::Duration::from_millis(5));

                if now.elapsed() > time::Duration::from_millis(200) {
                    now = time::Instant::now();
                    down(g)
                }
            }
        }
        view::draw_in_win(g, win);
    }
    while win.getch() != Some(pancurses::Input::Character('q')) {
        thread::sleep(time::Duration::from_millis(200));
    }
}

/// The entry point.
fn main() {
    const WIDTH: u8 = 10;
    const HEIGHT: u8 = 20;
    let win = view::init(WIDTH, HEIGHT);

    let piece_factory = PieceFactory::new();
    let b = Board::new(WIDTH, HEIGHT);
    let mut game = Game::new(b, piece_factory);
    win.nodelay(true);

    game_loop(&win, &mut game);

    view::end();
}

#[cfg(test)]
mod tests {}
