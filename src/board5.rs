use std::collections::VecDeque;
use std::collections::BTreeSet;
use std::str::FromStr;
use std::fmt;
use twiddle::Twiddle;
use enum_primitive::FromPrimitive;

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
            self.grid[point.x][point.y] = (piece as u16) << 13;
            Ok(())
        } else {
            return Err("Cannot place stone on top of existing stone.".into());
        }
    }

    fn add_piece(&mut self, point: &Point, piece: Piece) -> Result<(), String> {
        let spot = self.grid[point.x][point.y];
        let top_bits = top_piece_bits(spot);
        if top_bits == 0 {
            self.grid[point.x][point.y] = (piece as u16) << 13;
            return Ok(());
        }
        let mut top = Piece::from_u8(top_bits).unwrap();
        try!(piece.move_onto(&mut top));

        let mut lowest = spot.bits(1..0);
        let mut temp = spot.bits(11..2) >> 2;
        if top.owner() == Player::One {
            temp = temp | 0x400;
        } else {
            temp = temp | 0x800;
        }
        self.grid[point.x][point.y] = temp | ((piece as u16) << 13);
        if lowest == 0 {
            Ok(())
        } else {
            // Need to add the bumped piece to an existing continuation
            let my_spot = 0 as u16;
            for i in 0..7 {
                let cont = self.continuations[i];
                if cont.bits(15..11) == (point.x * 5 + point.y) as u16 {
                    let mut temp: u16 = cont & 0xF800; // Keep location
                    temp |= cont.bits(9..2); // Shift everything thats there
                    temp |= lowest << 9; // Add in the bumped piece
                    self.continuations[i] = temp;

                    lowest = cont & 3; // We may have to do this again...
                    if lowest == 0 {
                        return Ok(());
                    }
                }
            }

            // If we haven't returned yet, we need to start a new continuation
            for i in 0..7 {
                let cont = self.continuations[i];
                if cont.bits(15..11) == 0 {
                    // It's unclaimed - let's claim it by setting the location
                    let mut temp: u16 = ((point.x * 5 + point.y) as u16) << 11;
                    temp |= lowest << 9; // Add in the bumped piece
                    self.continuations[i] = temp;
                    return Ok(());
                }
            }

            // Need to figure out continuations
            Err("No available continuations. Did you play too many pieces?".into())
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
