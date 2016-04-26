use std::collections::VecDeque;
use std::collections::BTreeSet;
use std::str::FromStr;
use std::fmt;
use twiddle::Twiddle;
use enum_primitive::FromPrimitive;

use board::Board;
use board::PieceIter;
use board::PieceCount;
use board::board_from_str;
use board::str_from_board;
use piece::Piece;
use piece::Player;
use point::Point;

pub fn advance_piece_iterator(spot: &mut u16, extra: &mut [u16; 7])
                              -> Option<Piece> {
    for i in 7..0 {
        if extra[i].bits(15..11) != 0 {
            let piece = Piece::from_u8((((extra[i] & 1) << 2) | 1) as u8).unwrap();
            extra[i] = (extra[i] & 0xF800) | extra[i].bits(9..2);
            // We've shifted out everything from this continuation
            if extra[i] & 3 == 0 {
                extra[i] = 0;
            }
            return Some(piece);
        }
    }
    // Now the standard shift-out
    let temp = *spot;
    if temp & 3 != 0 {
        // Lower pieces
        let piece = Piece::from_u8((((temp & 1) << 2) | 1) as u8).unwrap();
        *spot = (temp & 0xF800) | temp.bits(11..2);
        Some(piece)
    } else if *spot != 0 {
        // Top piece
        let piece = Piece::from_u8(temp.bits(15..13) as u8).unwrap();
        *spot = 0;
        Some(piece)
    } else {
        None
    }
}

#[derive(Copy, Clone, Debug, RustcDecodable, RustcEncodable)]
pub struct Board5 {
    grid: [ [u16; 5]; 5],
    continuations: [u16; 7],
}

fn top_piece_bits(spot: u16) -> u8 {
    spot.bits(15..13) as u8
}

impl Board5 {
    // Return shifted location value
    fn location(p: &Point) -> u16 {
        ((p.x * 5 + p.y + 1) as u16) << 11
    }
}

impl Board for Board5 {
    fn new(board_size: usize) -> Board5 {
        assert!(board_size == 5);
        Board5 {
            grid: [[0; 5]; 5],
            continuations: [0; 7],
        }
    }


    fn at(&self, point: &Point) -> Result<PieceIter, &str> {
        let mut extra = self.continuations;
        let location = Board5::location(point);
        for i in 0..7 {
            if (extra[i] & location) != location {
                extra[i] = 0;
            } else if extra[i] != 0 {
                while (extra[i] & 3) == 0 {
                    extra[i] = (extra[i] & location) | extra[i].bits(9..2);
                }
            }
        }

        let mut spot = self.grid[point.x][point.y];
        if (spot & 0x0FFF) != 0 {
            while (spot & 3) == 0 {
                spot = (spot & 0xE000) | spot.bits(11..2);
            }
        }

        Ok(PieceIter::Board5Iter {
            spot: spot,
            extra: extra,
        })
    }

    fn at_reset(&mut self, point: &Point) -> Result<PieceIter, &str> {
        let mut extra = self.continuations;
        let location = Board5::location(point);
        for i in 0..7 {
            if extra[i] & location != location {
                extra[i] = 0;
            } else if extra[i] != 0 {
                self.continuations[i] = 0;
                while extra[i] & 3 == 0 {
                    extra[i] = (extra[i] & location) | extra[i].bits(9..2);
                }
            }
        }

        let mut spot = self.grid[point.x][point.y];
        self.grid[point.x][point.y] = 0;
        if (spot & 0x0FFF) != 0 {
            while spot & 3 == 0 {
                spot = (spot & 0xE000) | spot.bits(11..2);
            }
        }

        Ok(PieceIter::Board5Iter {
            spot: spot,
            extra: extra,
        })
    }

    fn size(&self) -> usize {
        5
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

    fn count(&self) -> PieceCount {
        let mut pieces = PieceCount::new(self.size());

        for row in self.grid.iter() {
            for spot in row.iter() {
                match Piece::from_u8(spot.bits(15..13) as u8) {
                    Some(Piece::OneFlat) => pieces.p1_flat += 1,
                    Some(Piece::OneStanding) => pieces.p1_flat += 1,
                    Some(Piece::OneCapstone) => pieces.p1_cap += 1,
                    Some(Piece::TwoFlat) => pieces.p2_flat += 1,
                    Some(Piece::TwoStanding) => pieces.p2_flat += 1,
                    Some(Piece::TwoCapstone) => pieces.p2_cap += 1,
                    None => {},
                }

                for i in 0..6 {
                    match spot.bits((2 * i + 1)..(2 * i)) {
                        1 => pieces.p1_flat += 1,
                        2 => pieces.p2_flat += 1,
                        _ => {},
                    }
                }
            }
        }
        //TODO: Count continuations
        pieces
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
        let mut temp = spot.bits(11..2);
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
            for i in 0..7 {
                let cont = self.continuations[i];
                if (cont & 0xF800) == Board5::location(point) {
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
                    let mut temp: u16 = Board5::location(point);
                    temp |= lowest << 9; // Add in the bumped piece
                    self.continuations[i] = temp;
                    return Ok(());
                }
            }

            // Need to figure out continuations
            Err("No available continuations. Did you play too many pieces?".into())
        }
    }
}

impl FromStr for Board5 {
    type Err=();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        board_from_str::<Board5>(s)
    }
}

impl fmt::Display for Board5 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        str_from_board(self, f)
    }
}
