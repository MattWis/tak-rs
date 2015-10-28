use std::mem;
use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
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
        // TODO: Stacking checks
        self.pieces.push(piece);
    }

    fn place_piece(&mut self, piece: Piece) -> () {
        assert!(self.pieces.len() == 0,
                "Cannot place stone on top of existing stone.");
        self.pieces.push(piece);
    }
}

enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl Direction {
    fn adjust(&self, x: usize, y: usize, offset: usize) -> (usize, usize) {
        match self {
            &Direction::Right => (x + offset, y),
            &Direction::Left => (x - offset, y),
            &Direction::Up => (x, y + offset),
            &Direction::Down => (x, y - offset),
        }
    }
}

enum Turn {
    Placement {
        x: usize,
        y: usize,
        piece: Piece,
    },
    Slide {
        x: usize,
        y: usize,
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
        let turn = chars.next().unwrap();
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

            Ok(Turn::Placement {
                x: x,
                y: y,
                piece: Piece {
                    owner: player,
                    stone: stone,
                },
            })
        } else if "RLUD".contains(turn) {
            let direction = match turn {
                'R' => Direction::Right,
                'L' => Direction::Left,
                'U' => Direction::Up,
                'D' => Direction::Down,
                _ => return Err(()),
            };
            let offsets: Vec<usize> = chars.map(|c| c.to_digit(10).unwrap() as usize).collect();

            Ok(Turn::Slide {
                x: x,
                y: y,
                direction: direction,
                offsets: offsets,
            })
        } else {
            Err(())
        }

    }
}

fn play(turn: &Turn, board: &mut Board) -> () {
    match turn {
        &Turn::Placement { x, y, piece } => {
            board.grid[x][y].place_piece(piece);
        }
        &Turn::Slide { x, y, ref direction, ref offsets } => {
            assert!(offsets.len() == board.grid[x][y].len(),
                    "Trying to move a different number of pieces than exist.");

            let cell = mem::replace(&mut board.grid[x][y], Cell::new());
            let points = offsets.iter().map(|z| direction.adjust(x, y, *z));
            for (point, piece) in points.zip(cell.pieces.iter()) {
                let (x, y) = point;
                board.grid[x][y].add_piece(*piece);
            }
        }
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
}

fn main() {
    let mut game = Board::new(4);
    play(&("a1S1".parse::<Turn>().unwrap()), &mut game);
    play(&("a2F2".parse::<Turn>().unwrap()), &mut game);
    play(&("a1U1".parse::<Turn>().unwrap()), &mut game);
    play(&("a2R12".parse::<Turn>().unwrap()), &mut game);
    println!("{:#?}", game);
    println!("{:#?}", "a1F1".parse::<Turn>());
    println!("{:#?}", "a1D1".parse::<Turn>());
}
