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

    fn follow(&self, paths: &mut VecDeque<Path>, dirs: &Vec<Direction>,
             player: Player) -> BTreeSet<Point> {
        let mut visited = BTreeSet::new();
        for path in paths.iter() {
            match path.walk(self.size()) {
                Some(p) => visited.insert(p),
                None => true,
            };
        }

        while let Some(path) = paths.pop_front() {
            let start = path.walk(self.size());
            println!("{:?}", start);
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

    pub fn check_winner(&self) -> Option<piece::Player> {
        let horiz_dirs = vec![Direction::Right, Direction::Down, Direction::Up];
        let mut paths = VecDeque::new();
        for y in 0..self.grid.len() {
            let point = Point { x: 0, y: y };
            paths.push_back(Path::new(point));
        }
        let visited = self.follow(&mut paths.clone(), &horiz_dirs, Player::One);
        if visited.iter().filter(|p| p.x == self.size() - 1)
                         .collect::<Vec<_>>().len() > 0 {
            return Some(piece::Player::One)
        }
        let visited = self.follow(&mut paths.clone(), &horiz_dirs, Player::Two);
        if visited.iter().filter(|p| p.x == self.size() - 1)
                         .collect::<Vec<_>>().len() > 0 {
            return Some(piece::Player::Two)
        }

        let vert_dirs = vec![Direction::Up, Direction::Right, Direction::Left];
        paths = VecDeque::new();
        for x in 0..self.grid.len() {
            let point = Point { x: x, y: 0 };
            paths.push_back(Path::new(point));
        }
        let visited = self.follow(&mut paths.clone(), &vert_dirs, Player::One);
        if visited.iter().filter(|p| p.y == self.size() - 1)
                         .collect::<Vec<_>>().len() > 0 {
            return Some(Player::One)
        }
        let visited = self.follow(&mut paths.clone(), &vert_dirs, Player::Two);
        if visited.iter().filter(|p| p.y == self.size() - 1)
                         .collect::<Vec<_>>().len() > 0 {
            return Some(Player::Two)
        }
        None
    }
}
