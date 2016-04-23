use std::collections::VecDeque;
use std::collections::BTreeSet;
use std::str::FromStr;
use std::iter;
use std::fmt;
use std::mem;

use piece::Stone;
use piece::Piece;
use piece::Player;
use piece;
use board5;
use point::Point;
use turn::Direction;

#[derive(Clone, Debug, RustcDecodable, RustcEncodable)]
pub struct Square {
    pub pieces: Vec<Piece>,
}

impl Square {
    pub fn new() -> Square {
        Square { pieces: vec![] }
    }

    pub fn len(&self) -> usize {
        self.pieces.len()
    }

    pub fn add_piece(&mut self, piece: Piece) -> Result<(), String> {
        match self.pieces.last_mut() {
            Some(base) => try!(piece.move_onto(base)),
            None => {}
        }
        self.pieces.push(piece);
        Ok(())
    }

    fn place_piece(&mut self, piece: Piece) -> Result<(), &str> {
        if self.len() != 0 {
            return Err("Cannot place stone on top of existing stone.");
        }
        self.pieces.push(piece);
        Ok(())
    }

    fn add_to_string(&self, string: &mut String, max_in_cell: usize) -> () {
        string.push_str("|");
        for piece in self.pieces.iter() {
            string.push_str(&(piece.to_string()));
        }
        let padding = max_in_cell - self.len();
        let space: String = iter::repeat("  ").take(padding).collect();
        string.push_str(&space);
    }

    // Used for moving stones eligibility
    pub fn mover(&self) -> Option<Player> {
        self.pieces.last().map(|piece| piece.owner())
    }

    // Used for road wins
    pub fn owner(&self) -> Option<Player> {
        self.pieces.last().and_then(|piece|
            if piece.stone() == Stone::Standing {
                None
            } else {
                Some(piece.owner())
            })
    }

    // Used for winning the flats
    pub fn scorer(&self) -> Option<Player> {
        self.pieces.last().and_then(|piece|
            if piece.stone() == Stone::Flat {
                Some(piece.owner())
            } else {
                None
            })
    }
}

#[derive(Copy, Clone, Debug, RustcDecodable, RustcEncodable)]
pub struct PieceCount {
    p1_flat: usize,
    p1_cap: usize,
    p2_flat: usize,
    p2_cap: usize,
    max_flat: usize,
    max_cap: usize,
}

impl PieceCount {
    fn new(size: usize) -> PieceCount {
        let flat_counts = [15, 20, 30, 40, 50];
        let capstone_counts = [0, 1, 1, 2, 2];
        PieceCount {
            p1_flat: 0,
            p1_cap: 0,
            p2_flat: 0,
            p2_cap: 0,
            max_flat: flat_counts[size - 4],
            max_cap: capstone_counts[size - 4],
        }
    }

    fn add(&mut self, piece: &Piece) {
        if piece.owner() == Player::One {
            if piece.stone() == Stone::Capstone {
                self.p1_cap += 1;
            } else {
                self.p1_flat += 1;
            }
        } else {
            if piece.stone() == Stone::Capstone {
                self.p2_cap += 1;
            } else {
                self.p2_flat += 1;
            }
        }
    }

    fn used_up(&self, piece: &Piece) -> bool {
        if piece.owner() == Player::One {
            if piece.stone() == Stone::Capstone {
                return self.p1_cap >= self.max_cap
            } else {
                return self.p1_flat >= self.max_flat
            }
        } else {
            if piece.stone() == Stone::Capstone {
                return self.p2_cap >= self.max_cap
            } else {
                return self.p2_flat >= self.max_flat
            }
        }
    }
}

#[derive(Clone, Debug, RustcDecodable, RustcEncodable)]
pub enum PieceIter {
    NaiveBoardIter {
        square: Square,
        index: usize,
    },
    Board5Iter {
        spot: u16,
        extra: [u16; 7],
    }
}

impl Iterator for PieceIter {
    type Item = Piece;

    fn next(&mut self) -> Option<Piece> {
        match *self {
            PieceIter::NaiveBoardIter { ref square, ref mut index } => {
                if *index >= square.pieces.len() {
                    return None;
                }
                let s = square.pieces[*index];
                *index = *index + 1;
                Some(s)
            }
            PieceIter::Board5Iter { ref mut spot, ref mut extra } => {
                board5::advance_piece_iterator(spot, extra)
            }
        }
    }
}

impl PieceIter {
    pub fn mover(&self) -> Option<Player> {
        let dup = self.clone();
        dup.last().map(|piece| piece.owner())
    }
}

pub trait Board {
    fn new(usize) -> Self;
    fn size(&self) -> usize;
    fn full(&self) -> bool;
    fn place_piece(&mut self, point: &Point, piece: Piece) -> Result<(), String>;
    fn add_piece(&mut self, point: &Point, piece: Piece) -> Result<(), String>;
    fn used_up(&self, piece: &Piece) -> bool;
    fn follow(&self,
              starts: &mut VecDeque<Point>,
              player: Player)
              -> BTreeSet<Point>;

    // These 2 aren't necessarily efficient
    fn at(&self, point: &Point) -> Result<PieceIter, &str>;
    fn at_reset(&mut self, point: &Point) -> Result<PieceIter, &str>;

    // I want these to die
    fn at_mut(&mut self, point: &Point) -> Result<&mut Square, &str>;
    fn squares(&self) -> Vec<&Square>;
}

#[derive(Clone, Debug, RustcDecodable, RustcEncodable)]
pub struct NaiveBoard {
    grid: Vec<Vec<Square>>,
    count: PieceCount,
}

impl fmt::Display for NaiveBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut full = String::new();
        let max = match self.grid
                            .iter()
                            .map(|r| r.iter().map(|c| c.len()).max())
                            .max() {
            Some(Some(x)) => x,
            _ => return Err(fmt::Error),
        };

        let width = (max * 2 + 1) * self.grid.len();
        full.push_str(&(iter::repeat("_").take(width).collect::<String>()));
        full.push_str("\n");
        for line in self.grid.iter().rev() {
            for cell in line.iter() {
                cell.add_to_string(&mut full, max);
            }
            full.push_str("\n");
        }
        try!(write!(f, "{}", full));

        try!(write!(f, "P1: {}/{} Flatstones\n", self.count.p1_flat, self.count.max_flat));
        try!(write!(f, "P1: {}/{} Capstones\n", self.count.p1_cap, self.count.max_cap));
        try!(write!(f, "P2: {}/{} Flatstones\n", self.count.p2_flat, self.count.max_flat));
        write!(f, "P2: {}/{} Capstones\n", self.count.p2_cap, self.count.max_cap)

    }
}

impl NaiveBoard {
    // Internal use only
    fn at_int(&self, point: &Point) -> Result<&Square, &str> {
        let row = try!(self.grid.get(point.y).ok_or("Invalid point"));
        row.get(point.x).ok_or("Invalid point")
    }
}

impl Board for NaiveBoard {
    fn new(board_size: usize) -> NaiveBoard {
        assert!(board_size >= 4 && board_size <= 8);
        NaiveBoard {
            grid: vec![vec![Square::new(); board_size]; board_size],
            count: PieceCount::new(board_size),
        }
    }

    fn at(&self, point: &Point) -> Result<PieceIter, &str> {
        let row = try!(self.grid.get(point.y).ok_or("Invalid point"));
        let cell = try!(row.get(point.x).ok_or("Invalid point"));
        Ok(PieceIter::NaiveBoardIter {
            square: cell.clone(),
            index: 0,
        })
    }

    fn at_mut(&mut self, point: &Point) -> Result<&mut Square, &str> {
        let row = try!(self.grid.get_mut(point.y).ok_or("Invalid point"));
        row.get_mut(point.x).ok_or("Invalid point")
    }

    fn at_reset(&mut self, point: &Point) -> Result<PieceIter, &str> {
        let row = try!(self.grid.get_mut(point.y).ok_or("Invalid point"));
        let square = try!(row.get_mut(point.x).ok_or("Invalid point"));
        Ok(PieceIter::NaiveBoardIter {
            square: mem::replace(square, Square::new()),
            index: 0,
        })
    }

    fn size(&self) -> usize {
        self.grid.len()
    }

    fn squares(&self) -> Vec<&Square> {
        self.grid.iter().flat_map(|row| row.iter()).collect()
    }

    /// Checks to see if all spaces have at least one piece
    fn full(&self) -> bool {
        !self.squares().iter().any(|sq| sq.pieces.is_empty())
    }

    fn place_piece(&mut self, point: &Point, piece: Piece) -> Result<(), String> {
        {
            let square = try!(self.at_mut(point));
            try!(square.place_piece(piece));
        }
        self.count.add(&piece);
        Ok(())
    }

    fn add_piece(&mut self, point: &Point, piece: Piece) -> Result<(), String> {
        {
            let square = try!(self.at_mut(point));
            try!(square.add_piece(piece));
        }
        Ok(())
    }

    fn used_up(&self, piece: &Piece) -> bool {
        self.count.used_up(piece)
    }

    fn follow(&self,
              starts: &mut VecDeque<Point>,
              player: Player)
              -> BTreeSet<Point> {
        let mut connected = BTreeSet::new();
        let mut visited = BTreeSet::new();

        while let Some(start) = starts.pop_front() {
            visited.insert(start);
            if self.at_int(&start).ok().and_then(|p| p.owner()) == Some(player) {
                connected.insert(start);
                for point in Direction::neighbors(&start, self.size()) {
                    if !visited.contains(&point) {
                        starts.push_back(point)
                    }
                }
            }
        }
        connected
    }
}

impl FromStr for NaiveBoard {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Pay a runtime cost for String slices to guarantee no panics
        fn slice(s: &str, start: usize, end: usize) -> String {
            s.chars().skip(start).take(end - start).collect::<String>()
        }

        // Parse the pieces of a non-empty square
        fn parse_square(s: &str) -> Result<Square, String> {
            let mut sq = Square::new();
            let mut i = 0;
            while i < s.chars().count() {
                let c = s.chars().nth(i);
                let stone = s.chars().nth(i+1);
                let piece = if c == Some('1') {
                    if stone == Some('S') {
                        i += 1;
                        Piece::new(Stone::Standing, Player::One)
                    } else if stone == Some('C') {
                        i += 1;
                        Piece::new(Stone::Capstone, Player::One)
                    } else {
                        Piece::new(Stone::Flat, Player::One)
                    }
                } else if c == Some('2') {
                    if stone == Some('S') {
                        i += 1;
                        Piece::new(Stone::Standing, Player::Two)
                    } else if stone == Some('C') {
                        i += 1;
                        Piece::new(Stone::Capstone, Player::Two)
                    } else {
                        Piece::new(Stone::Flat, Player::Two)
                    }
                } else {
                    return Err("Out of order stuff".into())
                };
                try!(sq.add_piece(piece));
                i += 1;
            }
            Ok(sq)
        }

        fn parse_row(s: &str) -> Result<Vec<Square>, String> {
            println!("{}", s);
            let mut v = vec![];
            for str in s.split(",") {
                let mut entry = if slice(str, 0, 1) == "x" {
                    match slice(str, 1, 2).parse::<usize>() {
                        Ok(num) => vec![Square::new(); num],
                        Err(_) => vec![Square::new()],
                    }
                } else if slice(str, 0, 1) == "1" || slice(str, 0, 1) == "2" {
                    let sq = try!(parse_square(str));
                    vec![sq]
                } else {
                    return Err("Empty cell should be marked with 'x'".into())
                };
                v.append(&mut entry);
            }
            Ok(v)
        }

        let size = s.chars().filter(|c| *c == '/').count() + 1;
        if size < 4 || size > 8 {
            return Err(());
        }
        let mut board = NaiveBoard::new(size);

        let iter = s.split("/");
        for (i, str) in iter.enumerate() {
            board.grid[size - i - 1] = match parse_row(&str) {
                Ok(x) => x,
                Err(_) => return Err(()),
            };
            if board.grid[size - i - 1].len() != size {
                return Err(());
            }
        }
        Ok(board)
    }
}
