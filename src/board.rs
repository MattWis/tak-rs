use std::collections::VecDeque;
use std::collections::BTreeSet;
use std::iter;
use std::fmt;

use piece::Piece;
use piece::Player;
use piece;
use point::Point;
use turn::Direction;

#[derive(Clone, Debug)]
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

    pub fn place_piece(&mut self, piece: Piece) -> Result<(), &str> {
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

    fn top_player(&self) -> Option<Player> {
        self.pieces.last().and_then(|piece|
            if piece.stone == piece::Stone::Standing {
                None
            } else {
                Some(piece.owner)
            })
    }
}

#[derive(Debug)]
pub struct Board {
    grid: Vec<Vec<Square>>,
}

impl fmt::Display for Board {
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
        write!(f, "{}", full)
    }
}

impl Board {
    pub fn new(board_size: usize) -> Board {
        assert!(board_size >= 4 && board_size <= 8);
        Board { grid: vec![vec![Square::new(); board_size]; board_size] }
    }

    pub fn at(&self, point: &Point) -> Result<&Square, &str> {
        let row = try!(self.grid.get(point.y).ok_or("Invalid point"));
        row.get(point.x).ok_or("Invalid point")
    }

    pub fn at_mut(&mut self, point: &Point) -> Result<&mut Square, &str> {
        let row = try!(self.grid.get_mut(point.y).ok_or("Invalid point"));
        row.get_mut(point.x).ok_or("Invalid point")
    }

    pub fn size(&self) -> usize {
        self.grid.len()
    }

    fn squares(&self) -> Vec<&Square> {
        self.grid.iter().flat_map(|row| row.iter()).collect()
    }

    pub fn is_top(&self, player: piece::Player, point: &Point) -> bool {
        self.at(point).ok()
            .and_then(|square| square.top_player())
            .map(|x| x == player)
            .unwrap_or(false)
    }

    /// Checks to see if all spaces have at least one piece
    fn full(&self) -> bool {
        !self.squares().iter().any(|sq| sq.pieces.is_empty())
    }

    fn follow(&self,
              starts: &mut VecDeque<Point>,
              player: Player)
              -> BTreeSet<Point> {
        let mut connected = BTreeSet::new();
        let mut visited = BTreeSet::new();

        while let Some(start) = starts.pop_front() {
            visited.insert(start);
            if self.is_top(player, &start) {
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

    /// Checks for the winner via a road win
    ///
    /// Uses follow to go from the right wall as far left as possible for each
    /// player, and then uses follow to go from the bottom wall as far up as
    /// possible. If the string of connected pieces reaches the far wall, it's
    /// a win.
    ///
    /// Returns when the first winner is found. It will give a weird (wrong?)
    /// answer when a move causes both players to "win". Is there a rule about
    /// that?
    fn check_road_winner(&self) -> Option<piece::Player> {
        let mut points = (0..self.size()).map(|y| Point { x: 0, y: y })
                                         .collect::<VecDeque<_>>();
        if self.follow(&mut points.clone(), Player::One)
               .iter()
               .any(|p| p.x == self.size() - 1) {
            return Some(piece::Player::One);
        }
        if self.follow(&mut points, Player::Two)
               .iter()
               .any(|p| p.x == self.size() - 1) {
            return Some(piece::Player::Two);
        }

        let mut points = (0..self.size()).map(|x| Point { x: x, y: 0 })
                                         .collect::<VecDeque<_>>();
        if self.follow(&mut points.clone(), Player::One)
               .iter()
               .any(|p| p.y == self.size() - 1) {
            return Some(Player::One);
        }
        if self.follow(&mut points, Player::Two)
               .iter()
               .any(|p| p.y == self.size() - 1) {
            return Some(Player::Two);
        }
        None
    }

    /// Counts total pieces of each type used
    fn piece_counts(&self) -> (u32, u32, u32, u32) {
        let mut p1_flat = 0;
        let mut p1_cap = 0;
        let mut p2_flat = 0;
        let mut p2_cap = 0;

        for piece in self.squares().iter().flat_map(|sq| sq.pieces.iter()) {
            if piece.owner == Player::One {
                if piece.stone == piece::Stone::Capstone {
                    p1_cap += 1;
                } else {
                    p1_flat += 1;
                }
            } else {
                if piece.stone == piece::Stone::Capstone {
                    p2_cap += 1;
                } else {
                    p2_flat += 1;
                }
            }
        }
        (p1_flat, p1_cap, p2_flat, p2_cap)
    }

    /// Checks for the winner via a flat win
    ///
    /// Counts the number of pieces laid, and if either player is out of
    /// pieces, then tallies the points to determine the winner
    pub fn check_flat_winner(&self) -> Option<piece::Player> {
        // Max number of each stones that can be had
        // Indexed with board size - 4
        let flat_counts = [15, 20, 30, 40, 50];
        let capstone_counts = [0, 1, 1, 2, 2];
        let flats = flat_counts[self.size() - 4];
        let caps = capstone_counts[self.size() - 4];

        let (p1_flat, p1_cap, p2_flat, p2_cap) = self.piece_counts();
        let pieces_empty = (flats <= p1_flat && caps <= p1_cap) ||
                           (flats <= p2_flat && caps <= p2_cap);

        if pieces_empty || self.full() {
            let mut p1_top = 0;
            let mut p2_top = 0;

            for square in self.squares().iter() {
                match square.top_player() {
                    Some(Player::One) => p1_top += 1,
                    Some(Player::Two) => p2_top += 1,
                    None => (),
                }
            }

            //Tie goes to p2 (rules claim ambiguity, I don't want ties)
            if p1_top > p2_top {
                return Some(Player::One);
            } else {
                return Some(Player::Two);
            }
        }
        None
    }

    pub fn check_winner(&self) -> Option<Player> {
        self.check_road_winner().or(self.check_flat_winner())
    }
}
