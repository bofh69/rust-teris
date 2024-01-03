// SPDX-FileCopyrightText: 2022 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: Apache-2.0

use rand::*;

#[derive(Clone)]
pub struct Tetramino {
    shape: [bool; 16],
    /// x, y
    offset: (i8, i8),
    /// width, height
    size: (u8, u8),
}

#[derive(Clone)]
pub enum PieceType {
    None,
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

#[derive(Clone)]
pub struct Piece {
    tetraminos: Vec<Tetramino>,
    index: u8,
    pub piece_type: PieceType,
}

pub struct Board {
    pub map: Vec<PieceType>,
    /// width, height
    pub size: (u8, u8),
}

pub struct Game {
    pub board: Board,
    pub piece: Piece,
    pub next_piece: Piece,
    piece_factory: PieceFactory,
    pub score: u32,
    pub lines: u16,
    pub pos: (i8, i8),
    pub last_tetris: bool,
    pub game_over: bool,
}

impl Tetramino {
    fn is_set(&self, x: i8, y: i8) -> bool {
        if !(0..=3).contains(&x) || !(0..=3).contains(&y) {
            return false;
        }
        self.shape[(x + y * 4) as usize]
    }
    fn width(&self) -> u8 {
        self.size.0
    }
    fn height(&self) -> u8 {
        self.size.1
    }
    fn new(tmpl: &str, offset_x: i8, offset_y: i8) -> Self {
        let mut s = tmpl.chars();
        let mut height: u8 = 0;
        let mut width: u8 = 0;
        let mut v = [false; 16];
        for y in 1..5 {
            for x in 1..5 {
                if let Some(c) = s.next() {
                    if c == 'X' {
                        v[x + y * 4 - 5_usize] = true;
                        height = y as u8;
                        if x as u8 > width {
                            width = x as u8;
                        }
                    }
                }
            }
        }
        Tetramino {
            shape: v,
            offset: (offset_x, offset_y),
            size: (width, height),
        }
    }
}

impl Piece {
    fn new(t: Vec<Tetramino>, pt: PieceType) -> Self {
        let mut t2: Vec<Tetramino> = vec![];
        for x in t {
            t2.push(Tetramino { ..x });
        }
        Piece {
            tetraminos: t2,
            index: 0,
            piece_type: pt,
        }
    }

    pub fn curr(&self) -> Tetramino {
        Tetramino {
            ..self.tetraminos[self.index as usize]
        }
    }

    pub fn next(&mut self) {
        self.index += 1;
        if self.index as usize >= self.tetraminos.len() {
            self.index = 0;
        }
    }
    pub fn prev(&mut self) {
        self.index = if self.index == 0 {
            (self.tetraminos.len() - 1) as u8
        } else {
            self.index - 1
        }
    }
}

impl Board {
    pub fn new(width: u8, height: u8) -> Self {
        Board {
            map: vec![PieceType::None; (width as usize) * height as usize],
            size: (width, height),
        }
    }

    // FIXME: Change to an iterator?
    fn get_indexes(&self, p: &Piece, x: i8, y: i8) -> Vec<usize> {
        let mut v = vec![];
        let curr = p.curr();
        let width = self.width() as usize;
        let x = x as usize - curr.offset.0 as usize;
        let y = y as usize - curr.offset.1 as usize;
        for y1 in 0..curr.height() {
            for x1 in 0..curr.width() {
                if curr.is_set(x1 as i8, y1 as i8) {
                    let x2 = x1 as usize + x;
                    let y2 = y1 as usize + y;
                    v.push(x2 + y2 * width);
                }
            }
        }
        v
    }

    pub fn draw(&mut self, p: &Piece, x: i8, y: i8) {
        for i in self.get_indexes(p, x, y) {
            self.map[i] = p.piece_type.clone();
        }
    }

    pub fn fits(&mut self, p: &Piece, x: i8, y: i8) -> bool {
        let c = p.curr();
        if x < c.offset.0 {
            return false;
        }
        if x + c.width() as i8 - c.offset.0 > self.width() as i8 {
            return false;
        }
        if y < c.offset.1 {
            return false;
        }
        if y + c.height() as i8 - c.offset.1 > self.height() as i8 {
            return false;
        }
        for i in self.get_indexes(p, x, y) {
            match self.map[i] {
                PieceType::None => (),
                _ => return false,
            }
        }
        true
    }

    pub fn clear(&mut self, p: &Piece, x: i8, y: i8) {
        for i in self.get_indexes(p, x, y) {
            self.map[i] = PieceType::None;
        }
    }

    pub fn width(&self) -> u8 {
        self.size.0
    }

    pub fn height(&self) -> u8 {
        self.size.1
    }

    pub fn is_set(&self, x: i8, y: i8) -> bool {
        !matches!(
            self.map[x as usize + y as usize * self.width() as usize],
            PieceType::None
        )
    }

    fn is_line_full(&self, y: i8) -> bool {
        for x in 0..self.width() {
            if !self.is_set(x as i8, y) {
                return false;
            }
        }
        true
    }

    fn clear_line(&mut self, y: u8) {
        let offset = y as usize * self.width() as usize;
        for x in 0..self.width() {
            self.map[x as usize + offset] = PieceType::None;
        }
    }

    fn copy_line(&mut self, from_y: i8, to_y: i8) {
        for x in 0..self.width() {
            let from_pos = x as usize + from_y as usize * self.width() as usize;
            let to_pos = x as usize + to_y as usize * self.width() as usize;
            self.map[to_pos] = self.map[from_pos].clone();
        }
    }

    fn scroll_down(&mut self, y: i8) {
        for y2 in 0..y {
            let ry = y - y2;
            self.copy_line(ry - 1, ry);
        }
        self.clear_line(0);
    }

    /// remove_full_lines() removes full lines and scrolls the rest.
    ///
    /// The fn returns the removed lines.
    pub fn remove_full_lines(&mut self) -> Vec<i8> {
        // RULES: This can be done in two ways:
        // This way, the original tetris way or
        // each unconnected region falls on its own. IE:
        //   YY
        // XXYXX
        // XX  X
        // With old algorith, one line is removed.
        // with the new, two lines are removed.
        let mut lines = 0;
        let mut v: Vec<i8> = vec![];
        let mut ry = self.height() as i8 - 1_i8;
        while ry >= 0 {
            if self.is_line_full(ry) {
                v.push(ry - lines);
                lines += 1;
                self.scroll_down(ry);
            } else {
                ry -= 1;
            }
        }
        v
    }

    #[cfg(test)]
    pub fn print(&self) {
        println!("Board:");
        let width = self.width() as usize;
        for y in 0..self.height() {
            print!(">");
            for x in 0..width {
                let o = match self.map[x + y as usize * width] {
                    PieceType::None => '.',
                    _ => 'X',
                };
                print!("{}", o);
            }
            println!("<");
        }
    }
}

fn get_pieces() -> Vec<Piece> {
    vec![
        Piece::new(vec![Tetramino::new("XX..XX..", 1, 1)], PieceType::O),
        Piece::new(
            vec![
                Tetramino::new("XXXX", 2, 0),
                Tetramino::new("X...X...X...X", 0, 2),
            ],
            PieceType::I,
        ),
        Piece::new(
            vec![
                Tetramino::new("XX...XX", 1, 1),
                Tetramino::new(".X..XX..X", 1, 1),
            ],
            PieceType::S,
        ),
        Piece::new(
            vec![
                Tetramino::new(".XX.XX", 1, 1),
                Tetramino::new("X...XX...X", 1, 1),
            ],
            PieceType::Z,
        ),
        Piece::new(
            vec![
                Tetramino::new("XXX..X..", 1, 1),
                Tetramino::new(".X..XX...X", 1, 1),
                Tetramino::new(".X..XXX", 1, 1),
                Tetramino::new("X...XX..X", 1, 1),
            ],
            PieceType::T,
        ),
        Piece::new(
            vec![
                Tetramino::new("X...X...XX", 1, 1),
                Tetramino::new("XXX.X", 1, 1),
                Tetramino::new("XX...X...X", 1, 1),
                Tetramino::new("..X.XXX", 1, 1),
            ],
            PieceType::L,
        ),
        Piece::new(
            vec![
                Tetramino::new(".X...X..XX", 1, 1),
                Tetramino::new("X...XXX", 1, 1),
                Tetramino::new("XX..X...X", 1, 1),
                Tetramino::new("XXX...X", 1, 1),
            ],
            PieceType::J,
        ),
    ]
}

pub struct PieceFactory {
    pieces: Vec<Piece>,
}

impl PieceFactory {
    pub fn new() -> PieceFactory {
        PieceFactory {
            pieces: get_pieces(),
        }
    }

    pub fn next(&self) -> Piece {
        let i = rand::thread_rng().gen_range(0..self.pieces.len());
        let p = &self.pieces[i];
        p.clone()
    }
}

impl Game {
    pub fn new(board: Board, piece_factory: PieceFactory) -> Game {
        let w = board.width() as i8;
        let p = piece_factory.next();
        let h = p.curr().offset.1;
        Game {
            board,
            piece: p,
            next_piece: piece_factory.next(),
            piece_factory,
            score: 0,
            lines: 0,
            pos: (w / 2, h),
            last_tetris: false,
            game_over: false,
        }
    }

    pub fn draw(&mut self) {
        self.board.draw(&self.piece, self.pos.0, self.pos.1);
    }

    pub fn clear(&mut self) {
        self.board.clear(&self.piece, self.pos.0, self.pos.1);
    }

    pub fn fits(&mut self) -> bool {
        self.board.fits(&self.piece, self.pos.0, self.pos.1)
    }

    pub fn turn_piece(&mut self) {
        self.piece.next();
    }

    pub fn counter_turn_piece(&mut self) {
        self.piece.prev();
    }

    pub fn piece_stuck(&mut self) -> Vec<i8> {
        let v = self.board.remove_full_lines();
        self.lines += v.len() as u16;
        self.score += 1 + 10 * ((v.len() * v.len()) as u32);
        self.piece = self.next_piece.clone();
        self.next_piece = self.piece_factory.next();
        self.pos = (self.board.width() as i8 / 2_i8, self.piece.curr().offset.1);
        if !self.fits() {
            self.game_over = true;
        }
        v
    }
}

#[cfg(test)]
mod tests {
    use crate::model::Board;
    use crate::model::Piece;
    use crate::model::Tetramino;

    fn get_i_piece() -> Piece {
        let p = crate::model::get_pieces();
        p[1].clone()
        /*
                ::Piece::new(vec![::Tetramino::new("XXXX", 2, 0),
                                  ::Tetramino::new("X...X...X...X...", 0, 2)],
                             ::PieceType::I)
        */
    }

    fn get_i() -> Tetramino {
        get_i_piece().curr()
    }

    fn get_rotated_i() -> Tetramino {
        let mut p = get_i_piece();
        p.next();
        p.curr()
    }

    fn assert_empty_line(b: &Board, y: i32) {
        for x in 0..b.width() {
            assert!(!b.is_set(x as i8, y as i8));
        }
    }

    #[test]
    fn piece_rotate() {
        let mut p = get_i_piece();
        let t1 = p.curr();
        p.next();
        let t2 = p.curr();
        assert!(t1.shape != t2.shape);
        p.next();
        let t2 = p.curr();
        assert_eq!(t1.shape, t2.shape);

        p.prev();
        let t2 = p.curr();
        assert!(t1.shape != t2.shape);

        p.prev();
        let t2 = p.curr();
        assert_eq!(t1.shape, t2.shape);
    }

    #[test]
    fn tet_is_set() {
        let i = get_i();
        assert!(i.is_set(0, 0));
        assert!(i.is_set(3, 0));
        assert!(!i.is_set(0, 1));
        assert!(!i.is_set(3, 3));

        let i = get_rotated_i();
        assert!(i.is_set(0, 0));
        assert!(i.is_set(0, 1));
        assert!(!i.is_set(1, 0));
        assert!(!i.is_set(3, 3));
    }

    #[test]
    fn tet_size() {
        let i = get_i();
        assert_eq!(i.height(), 1);
        assert_eq!(i.width(), 4);
    }

    #[test]
    fn tet_offset() {
        let i = get_i();
        assert_eq!(i.offset.0, 2);
        assert_eq!(i.offset.1, 0);
    }

    #[test]
    fn draw_on_board() {
        let i = get_i_piece();
        let mut b = Board::new(10, 20);

        b.draw(&i, 5, 0);
        b.print();

        for y in 1..b.height() {
            assert_empty_line(&b, i32::from(y));
        }
        for x in 0..3 {
            assert!(!b.is_set(x as i8, 0_i8));
        }
        for x in 3..7 {
            assert!(b.is_set(x as i8, 0_i8));
        }
        for x in 7..b.width() {
            assert!(!b.is_set(x as i8, 0_i8));
        }
        b.clear(&i, 5_i8, 0_i8);
        for y in 0..b.height() {
            assert_empty_line(&b, i32::from(y));
        }
    }

    fn add_almost_full_lines(b: &mut Board) {
        let mut i = get_i_piece();
        b.draw(&i, 5, 0);

        b.draw(&i, 6, 7);
        b.draw(&i, 2, 7);
        b.draw(&i, 2, 9);
        b.draw(&i, 6, 9);
        i.next();
        b.draw(&i, 8, 8);
    }

    fn add_full_lines(b: &mut Board) {
        add_almost_full_lines(b);

        let mut i = get_i_piece();
        i.next();
        b.draw(&i, 9, 8);
    }

    #[test]
    fn full_line_scrolls_rest() {
        let mut b = Board::new(10, 10);

        add_almost_full_lines(&mut b);

        let l = b.remove_full_lines();
        assert_eq!(l.len(), 0);

        let mut i = get_i_piece();
        i.next();
        b.draw(&i, 9, 8);

        b.print();

        let l = b.remove_full_lines();
        b.print();

        assert_eq!(l.len(), 2);
        assert!(l.contains(&9_i8));
        assert!(l.contains(&7_i8));

        let l = b.remove_full_lines();
        assert_eq!(l.len(), 0);

        for y in 0..2 {
            assert_empty_line(&b, y);
        }
        assert!(b.is_set(3, 2));
        for y in 3..8 {
            assert_empty_line(&b, y);
        }
    }

    #[test]
    fn new_game() {
        let b = Board::new(10, 20);
        let piece_factory = crate::PieceFactory::new();
        crate::Game::new(b, piece_factory);
    }

    #[test]
    fn piece_is_stuck() {
        let b = Board::new(10, 10);
        let piece_factory = crate::PieceFactory::new();
        let mut g = crate::Game::new(b, piece_factory);

        add_full_lines(&mut g.board);

        g.piece_stuck();
        g.board.print();
    }
}
