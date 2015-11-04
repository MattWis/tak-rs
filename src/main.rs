use std::mem;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Stone {
    Flat,
    Standing,
    Capstone,
}

#[derive(Clone, Copy, Debug)]
enum Player {
    One,
    Two,
}

#[derive(Clone, Copy, Debug)]
struct Piece {
    stone: Stone,
    owner: Player,
}

impl Piece {
    // Flatten a standing stone if a capstone moves onto it
    fn move_onto(&self, base: &mut Piece) -> () {
        assert!(base.stone != Stone::Standing || self.stone == Stone::Capstone,
                "Cannot move normal stone onto standing stone");
        if base.stone == Stone::Standing && self.stone == Stone::Capstone {
            base.stone = Stone::Flat;
        }
    }
}

impl FromStr for Piece {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 4 {
            return Err(());
        }
        let mut chars = s.chars();
        let turn = chars.nth(2).unwrap();
        if "FSC".contains(turn) {
            let stone = match turn {
                'F' => Stone::Flat,
                'S' => Stone::Standing,
                'C' => Stone::Capstone,
                _ => return Err(()),
            };
            let player = match chars.next() {
                Some('1') => Player::One,
                Some('2') => Player::Two,
                _ => return Err(()),
            };

            Ok(Piece {
                stone: stone,
                owner: player,
            })
        } else {
            Err(())
        }
    }

}
#[derive(Clone, Debug)]
struct Cell {
    pieces: Vec<Piece>,
}

impl Cell {
    fn new() -> Cell {
        Cell { pieces: vec![] }
    }

    fn len(&self) -> usize {
        self.pieces.len()
    }

    fn add_piece(&mut self, piece: Piece) -> () {
        match self.pieces.last_mut() {
            Some(base) => piece.move_onto(base),
            None => {}
        }
        self.pieces.push(piece);
    }

    fn place_piece(&mut self, piece: Piece) -> () {
        assert!(self.pieces.len() == 0,
                "Cannot place stone on top of existing stone.");
        self.pieces.push(piece);
    }
}

#[derive(Debug)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl Direction {
    fn adjust(&self, point: &Point, offset: usize) -> Point {
        match self {
            &Direction::Right => Point {
                x: point.x + offset,
                y: point.y,
            },
            &Direction::Left => Point {
                x: point.x - offset,
                y: point.y,
            },
            &Direction::Up => Point {
                x: point.x,
                y: point.y + offset,
            },
            &Direction::Down => Point {
                x: point.x,
                y: point.y - offset,
            },
        }
    }
}

#[derive(Debug)]
enum Turn {
    Placement {
        point: Point,
        piece: Piece,
    },
    Slide {
        point: Point,
        direction: Direction,
        offsets: Vec<usize>,
    },
}

impl FromStr for Turn {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 4 {
            return Err(());
        }
        let point = match s.parse::<Point>() {
            Ok(p) => p,
            Err(_) => return Err(()),
        };
        let mut chars = s.chars();
        let turn = chars.nth(2).unwrap();
        if let Ok(piece) = s.parse::<Piece>() {
            Ok(Turn::Placement {
                point: point,
                piece: piece,
            })
        } else {
            let direction = match turn {
                'R' => Direction::Right,
                'L' => Direction::Left,
                'U' => Direction::Up,
                'D' => Direction::Down,
                _ => return Err(()),
            };
            let offsets = chars.map(|c| c.to_digit(10).unwrap() as usize)
                               .collect();

            Ok(Turn::Slide {
                point: point,
                direction: direction,
                offsets: offsets,
            })
        }

    }
}

fn play(turn: &Turn, board: &mut Board) -> () {
    match turn {
        &Turn::Placement { ref point, ref piece } => {
            board.at(point).place_piece(*piece);
        }
        &Turn::Slide { ref point, ref direction, ref offsets } => {
            assert!(offsets.len() == board.at(point).len(),
                    "Trying to move a different number of pieces than exist.");

            let cell = mem::replace(board.at(point), Cell::new());
            let points = offsets.iter().map(|z| direction.adjust(point, *z));
            for (point, piece) in points.zip(cell.pieces.iter()) {
                board.at(&point).add_piece(*piece);
            }
        }
    }
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 {
            return Err(());
        }
        let mut chars = s.chars();
        let letters = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        let numbers = ['1', '2', '3', '4', '5', '6', '7', '8'];
        let grid_x = chars.next().unwrap();
        let grid_y = chars.next().unwrap();
        let x = match letters.iter().position(|c| *c == grid_x) {
            Some(num) => num,
            None => return Err(()),
        };
        let y = match numbers.iter().position(|c| *c == grid_y) {
            Some(num) => num,
            None => return Err(()),
        };
        Ok(Point { x: x, y: y })
    }

}

#[derive(Debug)]
struct Board {
    grid: Vec<Vec<Cell>>,
}

impl Board {
    fn new(board_size: usize) -> Board {
        Board { grid: vec![vec![Cell::new(); board_size]; board_size] }
    }

    fn at(&mut self, point: &Point) -> &mut Cell {
        &mut self.grid[point.x][point.y]
    }
}

#[test]
fn basic_placement() {
    let mut game = Board::new(4);
    play(&("a1S1".parse::<Turn>().unwrap()), &mut game);
    play(&("a2F2".parse::<Turn>().unwrap()), &mut game);
    play(&("d3C2".parse::<Turn>().unwrap()), &mut game);
}

#[test]
fn basic_movement() {
    let mut game = Board::new(4);
    play(&("a1S1".parse::<Turn>().unwrap()), &mut game);
    play(&("a2F2".parse::<Turn>().unwrap()), &mut game);
    play(&("a1U1".parse::<Turn>().unwrap()), &mut game);
    play(&("a2R12".parse::<Turn>().unwrap()), &mut game);
}

#[test]
#[should_panic]
fn invalid_movement() {
    let mut game = Board::new(4);
    play(&("a1S1".parse::<Turn>().unwrap()), &mut game);
    play(&("a2F2".parse::<Turn>().unwrap()), &mut game);
    play(&("a2D1".parse::<Turn>().unwrap()), &mut game);
}

#[test]
fn squash() {
    let mut game = Board::new(4);
    play(&("a1S1".parse::<Turn>().unwrap()), &mut game);
    play(&("a2C2".parse::<Turn>().unwrap()), &mut game);
    play(&("a2D1".parse::<Turn>().unwrap()), &mut game);
}


fn main() {
    let mut game = Board::new(4);
    play(&("a1S1".parse::<Turn>().unwrap()), &mut game);
    play(&("a2F2".parse::<Turn>().unwrap()), &mut game);
    play(&("a1U1".parse::<Turn>().unwrap()), &mut game);
    play(&("a2R12".parse::<Turn>().unwrap()), &mut game);
    println!("{:#?}", game);
}
