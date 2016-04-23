use std::collections::VecDeque;
use std::collections::BTreeSet;
use std::str::FromStr;
use std::fmt;
use twiddle::Twiddle;

use board::Board;
use board::Square;
use piece::Piece;
use piece::Player;
use piece;
use point::Point;

//TODO: Copy? (s needs to be removed first)
#[derive(Clone, Debug, RustcDecodable, RustcEncodable)]
pub struct Board5 {
    grid: [ [u16; 5]; 5],
    continuations: [u16; 7],
    s: Square, // This is just a stupid hack for compilation - 4/23/16

}

impl fmt::Display for Board5 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Not Implemented")
    }
}

impl Board for Board5 {
    fn new(board_size: usize) -> Board5 {
        assert!(board_size == 5);
        Board5 {
            grid: [[0; 5]; 5],
            continuations: [0; 7],
            s: Square::new(),
        }
    }


    fn at(&self, point: &Point) -> Result<&Square, &str> {
        Err("Not implemented")
    }

    fn at_mut(&mut self, point: &Point) -> Result<&mut Square, &str> {
        Err("Not implemented")
    }

    fn at_reset(&mut self, point: &Point) -> Result<Square, &str> {
        Err("Not implemented")
    }

    fn size(&self) -> usize {
        self.grid.len()
    }

    fn squares(&self) -> Vec<&Square> {
        vec![&self.s]
    }

    /// Checks to see if all spaces have at least one piece
    fn full(&self) -> bool {
        false
    }

    fn place_piece(&mut self, point: &Point, piece: Piece) -> Result<(), String> {
        if self.grid[point.x][point.y].bits(16..13) == 0 {
            self.grid[point.x][point.y] = (piece.to3bits() as u16) << 13;
            Ok(())
        } else {
            return Err("Cannot place stone on top of existing stone.".into());
        }
    }

    fn add_piece(&mut self, point: &Point, piece: Piece) -> Result<(), String> {
        Ok(())
    }

    fn used_up(&self, piece: &Piece) -> bool {
        false
    }

    fn follow(&self,
              starts: &mut VecDeque<Point>,
              player: Player)
              -> BTreeSet<Point> {
        BTreeSet::new()
    }
}

impl FromStr for Board5 {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Board5::new(5))
    }
}
