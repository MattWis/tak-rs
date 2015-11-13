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

    pub fn place_piece(&mut self, piece: Piece) -> Result<(), String> {
        if self.len() != 0 {
            return Err("Cannot place stone on top of existing stone.".into())
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
        match self.pieces.last() {
            Some(piece) => if piece.stone == piece::Stone::Standing {
                None
            } else {
                Some(piece.owner)
            },
            None => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Path {
    start: Point,
    steps: Vec<Direction>,
}

impl Path {
    fn new(start: Point) -> Path {
        Path { start: start, steps: vec![] }
    }

    fn walk(&self, size: usize) -> Option<Point> {
        let mut point = Some(self.start);
        for dir in self.steps.iter() {
            point = dir.adjust(&point, 1, size);
        }
        point
    }
}

#[derive(Debug)]
pub struct Board {
    grid: Vec<Vec<Square>>,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut full = String::new();
        let max = match self.grid.iter()
                                 .map(|r| r.iter().map(|c| c.len()).max())
                                 .max() {
            Some(Some(x)) => x,
            _ => return Err(fmt::Error)
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
        Board { grid: vec![vec![Square::new(); board_size]; board_size] }
    }

    pub fn at(&mut self, point: &Point) -> &mut Square {
        &mut self.grid[point.y][point.x]
    }

    pub fn size(&self) -> usize {
        self.grid.len()
    }

    pub fn is_top(&self, player: piece::Player, point: &Point) -> bool {
        match self.grid[point.y][point.x].top_player() {
            Some(x) => x == player,
            None => false,
        }
    }

    fn follow(&self, starts: &Vec<Point>, dirs: &Vec<Direction>, player: Player)
              -> BTreeSet<Point> {
        let mut visited = starts.iter().filter(|p| self.is_top(player, &p))
                                       .map(|p| *p).collect::<BTreeSet<_>>();
        let mut paths = visited.iter().map(|p| Path::new(*p))
                                      .collect::<VecDeque<_>>();

        while let Some(path) = paths.pop_front() {
            let start = path.walk(self.size());
            for dir in dirs.iter() {
                if let Some(p) = dir.adjust(&start, 1, self.size()) {
                    if !visited.contains(&p) && self.is_top(player, &p) {
                        let mut new_path = path.clone();
                        new_path.steps.push(*dir);
                        paths.push_back(new_path);
                        visited.insert(p);
                    }
                }
            }
        }
        visited
    }

    /// Checks for the winner
    ///
    /// Uses follow to go from the right wall as far left as possible for each
    /// player, and then uses follow to go from the bottom wall as far up as
    /// possible. If the string of connected pieces reaches the far wall, it's
    /// a win.
    ///
    /// Returns when the first winner is found. It will give a weird (wrong?)
    /// answer when a move causes both players to "win". Is there a rule about
    /// that?
    pub fn check_winner(&self) -> Option<piece::Player> {
        let dirs = vec![Direction::Right, Direction::Down, Direction::Up];
        let points = (0..self.size()).map(|y| Point { x: 0, y: y })
                                     .collect::<Vec<_>>();
        if self.follow(&points, &dirs, Player::One).iter()
               .any(|p| p.x == self.size() - 1) {
            return Some(piece::Player::One)
        }
        if self.follow(&points, &dirs, Player::Two).iter()
               .any(|p| p.x == self.size() - 1) {
            return Some(piece::Player::Two)
        }

        let dirs = vec![Direction::Up, Direction::Right, Direction::Left];
        let points = (0..self.size()).map(|x| Point { x: x, y: 0 })
                                     .collect::<Vec<_>>();
        if self.follow(&points, &dirs, Player::One).iter()
               .any(|p| p.y == self.size() - 1) {
            return Some(Player::One)
        }
        if self.follow(&points, &dirs, Player::Two).iter()
               .any(|p| p.y == self.size() - 1) {
            return Some(Player::Two)
        }
        None
    }
}
