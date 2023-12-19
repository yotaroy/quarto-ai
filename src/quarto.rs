use std::collections::HashSet;
use std::fmt;
use std::fmt::Formatter;

const SIZE: usize = 4;

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct Piece {
    color: Color,
    shape: Shape,
    height: Height,
    top: Top,
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}{}{}{})",
            self.color, self.shape, self.height, self.top
        )
    }
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
enum Color {
    Black,
    White,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Color::Black => write!(f, "B"),
            Color::White => write!(f, "W"),
        }
    }
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
enum Shape {
    Square,
    Circle,
}

impl fmt::Display for Shape {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Shape::Square => write!(f, "S"),
            Shape::Circle => write!(f, "C"),
        }
    }
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
enum Height {
    Tall,
    Short,
}

impl fmt::Display for Height {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Height::Tall => write!(f, "T"),
            Height::Short => write!(f, "S"),
        }
    }
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
enum Top {
    Flat,
    Hole,
}

impl fmt::Display for Top {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Top::Flat => write!(f, "F"),
            Top::Hole => write!(f, "H"),
        }
    }
}

pub struct State {
    turn: usize,
    unused_pieces: HashSet<Piece>,
    board: [[Option<Piece>; SIZE]; SIZE],
    active_player: usize,
    selected_piece: Option<Piece>,
}

impl State {
    pub fn new() -> Self {
        let mut unused_pieces = HashSet::new();
        for c in [Color::Black, Color::White] {
            for s in [Shape::Square, Shape::Circle] {
                for h in [Height::Tall, Height::Short] {
                    for t in [Top::Flat, Top::Hole] {
                        unused_pieces.insert(Piece {
                            color: c,
                            shape: s,
                            height: h,
                            top: t,
                        });
                    }
                }
            }
        }
        State {
            turn: 0,
            unused_pieces,
            board: [[None; SIZE]; SIZE],
            active_player: 0,
            selected_piece: None,
        }
    }

    pub fn legal_placements(&self) -> Vec<(usize, usize)> {
        let mut placements = Vec::new();
        for h in 0..SIZE {
            for w in 0..SIZE {
                if self.board[h][w].is_none() {
                    placements.push((h, w));
                }
            }
        }
        placements
    }

    pub fn legal_pieces(&self) -> &HashSet<Piece> {
        &self.unused_pieces
    }

    pub fn put_piece(&mut self, h: usize, w: usize) {
        self.board[h][w] = self.selected_piece;
        self.selected_piece = None;
    }

    pub fn select_piece(&mut self, piece: Piece) {
        self.unused_pieces.remove(&piece);
        self.selected_piece = Some(piece);
        self.turn += 1;
        self.active_player = self.active_player ^ 1;
    }

    pub fn can_win(&self) -> bool {
        for i in 0..SIZE {
            if Self::have_common_attribute([
                self.board[i][0],
                self.board[i][1],
                self.board[i][2],
                self.board[i][3],
            ]) {
                return true;
            }
            if Self::have_common_attribute([
                self.board[0][i],
                self.board[1][i],
                self.board[2][i],
                self.board[3][i],
            ]) {
                return true;
            }
        }
        if Self::have_common_attribute([
            self.board[0][0],
            self.board[1][1],
            self.board[2][2],
            self.board[3][3],
        ]) {
            return true;
        }
        if Self::have_common_attribute([
            self.board[0][3],
            self.board[1][2],
            self.board[2][1],
            self.board[3][0],
        ]) {
            return true;
        }

        false
    }

    fn have_common_attribute(pieces: [Option<Piece>; SIZE]) -> bool {
        let mut color = HashSet::new();
        let mut shape = HashSet::new();
        let mut height = HashSet::new();
        let mut top = HashSet::new();
        for piece in pieces {
            if piece.is_none() {
                return false;
            }
            let piece = piece.unwrap();
            color.insert(piece.color);
            shape.insert(piece.shape);
            height.insert(piece.height);
            top.insert(piece.top);
        }
        color.len() == 1 || shape.len() == 1 || height.len() == 1 || top.len() == 1
    }

    pub fn print(&self) {
        println!("turn: {}", self.turn);
        if self.selected_piece.is_some() {
            println!("selected piece: {}", self.selected_piece.unwrap());
        }
        print!("unused pieces: {}\t", self.unused_pieces.len());
        for piece in &self.unused_pieces {
            print!("{} ", piece);
        }
        println!();
        println!("{}", self);
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for h in 0..SIZE {
            writeln!(f, "+--------+--------+--------+--------+")?;
            for w in 0..SIZE {
                write!(f, "| ");
                match self.board[h][w] {
                    Some(ref piece) => write!(f, "{}", piece)?,
                    None => write!(f, "(    )")?,
                }
                write!(f, " ");
            }
            writeln!(f, "|")?;
        }
        writeln!(f, "+--------+--------+--------+--------+")?;
        Ok(())
    }
}
