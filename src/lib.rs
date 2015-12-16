extern crate rustc_serialize;
use std::collections::VecDeque;
use std::fmt;
use std::mem;

pub mod piece;
pub mod point;
pub mod turn;
pub mod board;

pub use turn::Turn;
pub use turn::Direction;
pub use board::Board;
pub use piece::Player;
pub use piece::Stone;
pub use piece::Piece;
pub use point::Point;

#[derive(Clone, Debug, RustcDecodable, RustcEncodable)]
pub struct Game {
    board: Board,
    next: Player,
    history: Vec<Turn>,
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}\n", self.board));

        match self.check_winner() {
            Some(Player::One) => write!(f, "\nPlayer 1 Wins!"),
            Some(Player::Two) => write!(f, "\nPlayer 2 Wins!"),
            None => write!(f, ""),
        }
    }
}

impl Game {
    pub fn new(size: usize) -> Game {
        Game {
            board: Board::new(size),
            next: Player::One,
            history: vec![],
        }
    }


    fn check_winner(&self) -> Option<Player> {
        self.check_road_winner().or(self.check_flat_winner())
    }
    pub fn size(&self) -> usize {
        self.board.size()
    }

    pub fn to_string(&self) -> String {
        self.board.to_string()
    }

    pub fn play(&mut self, turn: &str, player: Player, owner: Option<Player>) -> Result<Option<Player>, String> {
        if self.next != player {
            return Err("Not your turn".into());
        }
        match turn.parse::<Turn>() {
            Ok(t) => self.play_parsed(t, owner),
            Err(_) => Err("Invalid move".into()),
        }
    }

    pub fn play_parsed(&mut self, turn: Turn, owner: Option<Player>) -> Result<Option<Player>, String> {
        match turn {
            Turn::Place { ref point, ref stone } => {
                match owner {
                    Some(player) => try!(self.place(point, stone, &player)),
                    None => return Err("Must supply owner to place piece".into()),
                }
            }
            Turn::Slide { ref num_pieces, ref point, ref direction, ref drops } => {
                try!(self.slide(num_pieces, point, direction, drops));
            }
        }
        self.history.push(turn);
        self.next = self.next.other();
        Ok(self.check_winner())
    }

    fn place(&mut self, point: &Point, stone: &Stone, owner: &Player) -> Result<(), String> {
        if self.history.len() >= 2 {
            if self.next != *owner {
                return Err("Player must play own piece".into())
            }
        } else if self.next == *owner {
            return Err("Play opposite piece on first turn".into())
        } else if *stone != piece::Stone::Flat {
            return Err("Play flat piece on first turn".into())
        }
        let piece = Piece { stone: *stone, owner: *owner };
        if self.board.used_up(&piece) {
            return Err("Player has used all of that type of stone".into())
        }
        self.board.place_piece(point, piece)
    }

    fn slide(&mut self, num_pieces: &usize, point: &Point, dir: &Direction, drops: &Vec<usize>) -> Result<(), String> {
        let cell = {
            let square = try!(self.board.at_mut(point));
            if *num_pieces > square.len() {
                return Err("Trying to move more pieces than exist".into());
            }
            if square.mover() != Some(self.next) {
                return Err("Must have control to move pile".into())
            }

            mem::replace(square, board::Square::new())
        };

        // Enforce carry limit
        if *num_pieces > self.size() {
            return Err("Cannot move more than the carry limit".into());
        }

        if drops.iter().fold(0, |sum, x| sum + x) != *num_pieces {
            return Err("Number of pieces claimed to move is diffent from number of pieces moved".into());
        }

        let size = self.size();
        let points = (0..).map(|x: usize| dir.adjust(point, x, size));

        let first_drop = [cell.len() - *num_pieces];
        let to_drop = first_drop.iter().chain(drops);
        let mut pieces = cell.pieces.iter();

        for (point, count) in points.zip(to_drop) {
            let p = match point {
                Some(x) => x,
                None => return Err("".into()),
            };
            let square = try!(self.board.at_mut(&p));
            for _ in (0..*count) {
                match pieces.next() {
                    Some(piece) => try!(square.add_piece(*piece)),
                    None => return Err("Used all pieces".into()),
                }
            }
        }
        Ok(())
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
        if self.board.follow(&mut points.clone(), Player::One)
               .iter()
               .any(|p| p.x == self.size() - 1) {
            return Some(piece::Player::One);
        }
        if self.board.follow(&mut points, Player::Two)
               .iter()
               .any(|p| p.x == self.size() - 1) {
            return Some(piece::Player::Two);
        }

        let mut points = (0..self.size()).map(|x| Point { x: x, y: 0 })
                                         .collect::<VecDeque<_>>();
        if self.board.follow(&mut points.clone(), Player::One)
               .iter()
               .any(|p| p.y == self.size() - 1) {
            return Some(Player::One);
        }
        if self.board.follow(&mut points, Player::Two)
               .iter()
               .any(|p| p.y == self.size() - 1) {
            return Some(Player::Two);
        }
        None
    }

    /// Checks for the winner via a flat win
    ///
    /// Counts the number of pieces laid, and if either player is out of
    /// pieces, then tallies the points to determine the winner
    pub fn check_flat_winner(&self) -> Option<piece::Player> {
        let used = (self.board.used_up(&Piece::new(Stone::Flat, Player::One)) &&
                    self.board.used_up(&Piece::new(Stone::Capstone, Player::One))) ||
                   (self.board.used_up(&Piece::new(Stone::Flat, Player::One)) &&
                    self.board.used_up(&Piece::new(Stone::Capstone, Player::One)));

        if used || self.board.full() {
            let mut p1_top = 0;
            let mut p2_top = 0;

            for square in self.board.squares().iter() {
                match square.scorer() {
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
}
