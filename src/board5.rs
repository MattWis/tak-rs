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

fn top_piece_bits(spot: u16) -> u8 {
    spot.bits(15..13) as u8
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
        5
    }

    fn squares(&self) -> Vec<&Square> {
        vec![&self.s]
    }

    /// Checks to see if all spaces have at least one piece
    fn full(&self) -> bool {
        for row in self.grid.iter() {
            for square in row.iter() {
                if square.bits(15..13) == 0 {
                    return false
                }
            }
        }
        true
    }

    fn place_piece(&mut self, point: &Point, piece: Piece) -> Result<(), String> {
        if top_piece_bits(self.grid[point.x][point.y]) == 0 {
            self.grid[point.x][point.y] = (piece.to3bits() as u16) << 13;
            Ok(())
        } else {
            return Err("Cannot place stone on top of existing stone.".into());
        }
    }

    fn add_piece(&mut self, point: &Point, piece: Piece) -> Result<(), String> {
        let spot = self.grid[point.x][point.y];
        let piece_bits = piece.to3bits() as u16;
        let top_bits = top_piece_bits(spot);
        if top_bits == 0 {
            self.grid[point.x][point.y] = piece_bits << 13;
            return Ok(());
        }
        let mut top = Piece::from3bits(top_bits);
        try!(piece.move_onto(&mut top));

        if spot.bits(1..0) == 0 {
            let mut temp = spot.bits(11..2) >> 2;
            if top.owner == Player::One {
                temp = temp | 0x400;
            } else {
                temp = temp | 0x800;
            }
            self.grid[point.x][point.y] = temp | (piece_bits << 13);
            Ok(())
        } else {
            // Need to figure out continuations
            Err("Not implemented".into())
        }
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
