use std::collections::VecDeque;
use std::collections::BTreeSet;
use std::fmt;
use std::iter;

use piece::Stone;
use piece::Piece;
use piece::Player;
use board5;
use point::Point;
use turn::Direction;

#[derive(Copy, Clone, Debug, RustcDecodable, RustcEncodable)]
pub struct PieceCount {
    pub p1_flat: usize,
    pub p1_cap: usize,
    pub p2_flat: usize,
    pub p2_cap: usize,
    pub max_flat: usize,
    pub max_cap: usize,
}

impl PieceCount {
    pub fn new(size: usize) -> PieceCount {
        let flat_counts = [15, 21, 30, 40, 50];
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

    pub fn add(&mut self, piece: &Piece) {
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

    pub fn used_up(&self, piece: &Piece) -> bool {
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
        square: Vec<Piece>,
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
                if *index >= square.len() {
                    return None;
                }
                let s = square[*index];
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

    pub fn owner(&self) -> Option<Player> {
        let dup = self.clone();
        dup.last().and_then(|piece|
            if piece.stone() == Stone::Standing {
                None
            } else {
                Some(piece.owner())
            })
    }

    // Used for winning the flats
    pub fn scorer(&self) -> Option<Player> {
        let dup = self.clone();
        dup.last().and_then(|piece|
            if piece.stone() == Stone::Flat {
                Some(piece.owner())
            } else {
                None
            })
    }
}

pub trait Board {
    fn new(usize) -> Self;
    fn size(&self) -> usize;
    fn full(&self) -> bool {
        !self.squares().iter().any(|it| it.clone().next() == None)
    }
    fn place_piece(&mut self, point: &Point, piece: Piece) -> Result<(), String>;
    fn add_piece(&mut self, point: &Point, piece: Piece) -> Result<(), String>;
    fn count(&self) -> PieceCount;
    fn follow(&self,
              starts: &mut VecDeque<Point>,
              player: Player)
              -> BTreeSet<Point> {
        let mut connected = BTreeSet::new();
        let mut visited = BTreeSet::new();

        while let Some(start) = starts.pop_front() {
            visited.insert(start);
            if self.at(&start).ok().and_then(|p| p.owner()) == Some(player) {
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

    // These 2 aren't necessarily efficient
    fn at(&self, point: &Point) -> Result<PieceIter, &str>;
    fn at_reset(&mut self, point: &Point) -> Result<PieceIter, &str>;
    fn squares(&self) -> Vec<PieceIter> {
        let mut v = Vec::new();
        for x in 0..self.size() {
            for y in 0..self.size() {
                v.push(self.at(&Point::new(x, y)).unwrap());
            }
        }
        v
    }
}

fn parse_square<T: Board>(s: &str, b: &mut T, point: &Point)
                          -> Result<(), String> {
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
        try!(b.add_piece(point, piece));
        i += 1;
    }
    Ok(())
}

fn parse_row<T: Board>(s: &str, b: &mut T, y: usize)
                       -> Result<(), String> {
    // Pay a runtime cost for String slices to guarantee no panics
    fn slice(s: &str, start: usize, end: usize) -> String {
        s.chars().skip(start).take(end - start).collect::<String>()
    }
    println!("{}", s);
    let mut index = 0;
    for str in s.split(",") {
        if slice(str, 0, 1) == "x" {
            match slice(str, 1, 2).parse::<usize>() {
                Ok(num) => index += num,
                Err(_) => index += 1,
            }
        } else if slice(str, 0, 1) == "1" || slice(str, 0, 1) == "2" {
            try!(parse_square(str, b, &Point::new(index, y)));
        } else {
            return Err("Empty cell should be marked with 'x'".into())
        };
    }
    Ok(())
}

pub fn board_from_str<T: Board>(s: &str) -> Result<T, ()> {
    let size = s.chars().filter(|c| *c == '/').count() + 1;
    if size != 5 {
        return Err(());
    }
    let mut board = T::new(size);

    let iter = s.split("/");
    for (i, str) in iter.enumerate() {
        match parse_row(&str, &mut board, 4 - i) {
            Ok(x) => x,
            Err(_) => return Err(()),
        };
    }
    Ok(board)
}

pub fn str_from_board<T: Board>(b: &T, f: &mut fmt::Formatter) -> fmt::Result {
    let mut full = String::new();
    let max = b.squares().iter().map(|x| x.clone().count())
               .max().unwrap_or(0);

    let width = (max * 2 + 1) * b.size();
    full.push_str(&(iter::repeat("_").take(width).collect::<String>()));
    full.push_str("\n");
    for y in 0..b.size() {
        for x in 0..b.size() {
            full.push_str("|");
            let iter = b.at(&Point::new(x,b.size() - 1 - y)).unwrap();
            let len = iter.clone().count();
            for piece in iter {
                full.push_str(&(piece.to_string()));
            }
            let space: String = iter::repeat("  ").take(max - len).collect();
            full.push_str(&space);
        }
        full.push_str("\n");
    }

    try!(write!(f, "{}", full));

    let count = b.count();
    try!(write!(f, "P1: {}/{} Flatstones\n", count.p1_flat, count.max_flat));
    try!(write!(f, "P1: {}/{} Capstones\n", count.p1_cap, count.max_cap));
    try!(write!(f, "P2: {}/{} Flatstones\n", count.p2_flat, count.max_flat));
    write!(f, "P2: {}/{} Capstones\n", count.p2_cap, count.max_cap)
}
