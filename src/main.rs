use std::mem;

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
    Placement { x: 0, y: 0, piece: Piece { stone: Stone::Standing, owner: Player::One }}.play(&mut game);
    Placement { x: 0, y: 1, piece: Piece { stone: Stone::Standing, owner: Player::Two }}.play(&mut game);
    Slide { x: 0, y: 0, direction: Direction::Up, offsets: vec![1]}.play(&mut game);
    Slide { x: 0, y: 1, direction: Direction::Right, offsets: vec![1, 2]}.play(&mut game);
    println!("{:#?}", game);
}
