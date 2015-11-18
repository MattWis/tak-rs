use std::collections::VecDeque;
use std::mem;

pub mod piece;
pub mod point;
pub mod turn;
pub mod board;

pub use turn::Turn;
pub use turn::Direction;
pub use board::Board;
pub use piece::Player;
pub use piece::Piece;
pub use point::Point;

pub struct Game {
    board: Board,
    next: Player,
    history: Vec<Turn>,
}

impl Game {
    pub fn new(size: usize) -> Game {
        Game {
            board: Board::new(size),
            next: Player::One,
            history: vec![],
        }
    }


    pub fn check_winner(&self) -> Option<Player> {
        self.check_road_winner().or(self.check_flat_winner())
    }
    pub fn size(&self) -> usize {
        self.board.size()
    }

    pub fn to_string(&self) -> String {
        self.board.to_string()
    }

    pub fn play(&mut self, turn: &str) -> Result<(), String> {
        match turn.parse::<Turn>() {
            Ok(t) => self.play_parsed(t),
            Err(_) => Err("Invalid move".into()),
        }
    }

    pub fn play_parsed(&mut self, turn: Turn) -> Result<(), String> {
        match turn {
            Turn::Place { ref point, ref piece } => {
                try!(self.place(point, piece));
            }
            Turn::Slide { ref point, ref direction, ref offsets } => {
                try!(self.slide(point, direction, offsets));
            }
        }
        self.history.push(turn);
        self.next = self.next.other();
        Ok(())
    }

    fn place(&mut self, point: &Point, piece: &Piece) -> Result<(), String> {
        if self.history.len() >= 2 {
            if self.next != piece.owner {
                return Err("Player must play own piece".into())
            }
        } else if self.next == piece.owner {
            return Err("Play opposite piece on first turn".into())
        } else if piece.stone != piece::Stone::Flat {
            return Err("Play flat piece on first turn".into())
        }
        if self.board.used_up(piece) {
            return Err("Player has used all of that type of stone".into())
        }
        let square = try!(self.board.at_mut(point));
        try!(square.place_piece(*piece));
        Ok(())
    }

    fn slide(&mut self, point: &Point, dir: &Direction, offsets: &Vec<usize>) -> Result<(), String> {
        let cell = {
            let square = try!(self.board.at_mut(point));
            if offsets.len() != square.len() {
                return Err("Trying to move a different number of pieces than exist".into());
            }
            if square.mover() != Some(self.next) {
                return Err("Must have control to move pile".into())
            }

            mem::replace(square, board::Square::new())
        };
        let points = offsets.iter()
                            .map(|z| dir.adjust(point, *z, self.size()))
                            .collect::<Vec<_>>();

        for (point, piece) in points.iter().zip(cell.pieces.iter()) {
            let p = match *point {
                Some(x) => x,
                None => return Err("".into()),
            };
            let square = try!(self.board.at_mut(&p));
            try!(square.add_piece(*piece));
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
        let (flats, caps) = self.board.piece_limits();
        let (p1_flat, p1_cap, p2_flat, p2_cap) = self.board.piece_counts();
        let pieces_empty = (flats <= p1_flat && caps <= p1_cap) ||
                           (flats <= p2_flat && caps <= p2_cap);

        if pieces_empty || self.board.full() {
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
