use std::collections::VecDeque;
use std::collections::BTreeSet;
use std::iter;
use std::fmt;

use piece::Stone;
use piece::Piece;
use piece::Player;
use piece;
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
        self.pieces.last().map(|piece| piece.owner)
    }

    // Used for road wins
    pub fn owner(&self) -> Option<Player> {
        self.pieces.last().and_then(|piece|
            if piece.stone == piece::Stone::Standing {
                None
            } else {
                Some(piece.owner)
            })
    }

    // Used for winning the flats
    pub fn scorer(&self) -> Option<Player> {
        self.pieces.last().and_then(|piece|
            if piece.stone == piece::Stone::Flat {
                Some(piece.owner)
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
        if piece.owner == Player::One {
            if piece.stone == Stone::Capstone {
                self.p1_cap += 1;
            } else {
                self.p1_flat += 1;
            }
        } else {
            if piece.stone == Stone::Capstone {
                self.p2_cap += 1;
            } else {
                self.p2_flat += 1;
            }
        }
    }

    fn used_up(&self, piece: &Piece) -> bool {
        if piece.owner == Player::One {
            if piece.stone == Stone::Capstone {
                return self.p1_cap >= self.max_cap
            } else {
                return self.p1_flat >= self.max_flat
            }
        } else {
            if piece.stone == Stone::Capstone {
                return self.p2_cap >= self.max_cap
            } else {
                return self.p2_flat >= self.max_flat
            }
        }
    }
}

#[derive(Clone, Debug, RustcDecodable, RustcEncodable)]
pub struct Board {
    grid: Vec<Vec<Square>>,
    count: PieceCount,
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
        try!(write!(f, "{}", full));

        try!(write!(f, "P1: {}/{} Flatstones\n", self.count.p1_flat, self.count.max_flat));
        try!(write!(f, "P1: {}/{} Capstones\n", self.count.p1_cap, self.count.max_cap));
        try!(write!(f, "P2: {}/{} Flatstones\n", self.count.p2_flat, self.count.max_flat));
        write!(f, "P2: {}/{} Capstones\n", self.count.p2_cap, self.count.max_cap)

    }
}

impl Board {
    pub fn new(board_size: usize) -> Board {
        assert!(board_size >= 4 && board_size <= 8);
        Board {
            grid: vec![vec![Square::new(); board_size]; board_size],
            count: PieceCount::new(board_size),
        }
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

    pub fn squares(&self) -> Vec<&Square> {
        self.grid.iter().flat_map(|row| row.iter()).collect()
    }

    /// Checks to see if all spaces have at least one piece
    pub fn full(&self) -> bool {
        !self.squares().iter().any(|sq| sq.pieces.is_empty())
    }

    pub fn place_piece(&mut self, point: &Point, piece: Piece) -> Result<(), String> {
        {
            let square = try!(self.at_mut(point));
            try!(square.place_piece(piece));
        }
        self.count.add(&piece);
        Ok(())
    }

    pub fn used_up(&self, piece: &Piece) -> bool {
        self.count.used_up(piece)
    }

    pub fn follow(&self,
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
}
