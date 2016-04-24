use std::collections::VecDeque;
use std::collections::BTreeSet;
use std::str::FromStr;
use std::iter;
use std::fmt;
use std::mem;

use piece::Stone;
use piece::Piece;
use piece::Player;
use board::Board;
use board::PieceIter;
use board::PieceCount;
use board::board_from_str;
use board::str_from_board;
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

#[derive(Clone, Debug, RustcDecodable, RustcEncodable)]
pub struct NaiveBoard {
    grid: Vec<Vec<Square>>,
    count: PieceCount,
}

impl NaiveBoard {
    // Internal use only
    fn at_int(&self, point: &Point) -> Result<&Square, &str> {
        let row = try!(self.grid.get(point.y).ok_or("Invalid point"));
        row.get(point.x).ok_or("Invalid point")
    }

    fn at_mut(&mut self, point: &Point) -> Result<&mut Square, &str> {
        let row = try!(self.grid.get_mut(point.y).ok_or("Invalid point"));
        row.get_mut(point.x).ok_or("Invalid point")
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
            square: cell.pieces.clone(),
            index: 0,
        })
    }

    fn at_reset(&mut self, point: &Point) -> Result<PieceIter, &str> {
        let row = try!(self.grid.get_mut(point.y).ok_or("Invalid point"));
        let square = try!(row.get_mut(point.x).ok_or("Invalid point"));
        Ok(PieceIter::NaiveBoardIter {
            square: mem::replace(square, Square::new()).pieces,
            index: 0,
        })
    }

    fn size(&self) -> usize {
        self.grid.len()
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

    fn count(&self) -> PieceCount {
        self.count
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
    type Err=();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        board_from_str::<NaiveBoard>(s)
    }
}

impl fmt::Display for NaiveBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        str_from_board(self, f)
    }
}
