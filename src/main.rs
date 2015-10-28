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

trait Turn {
    fn play(&self, &mut Board) -> ();
}

struct Placement {
    x: usize,
    y: usize,
    piece: Piece,
}

impl Turn for Placement {
    fn play(&self, board: &mut Board) -> () {
        board.grid[self.x][self.y].place_piece(self.piece);
    }
}

struct Slide {
    x: usize,
    y: usize,
    direction: Direction,
    offsets: Vec<usize>,
}

impl Slide {
    fn locations(&self) -> Vec<(usize, usize)> {
        let mut points = Vec::<(usize, usize)>::new();
        for offset in self.offsets.iter() {
            points.push(self.direction.adjust(self.x, self.y, *offset))
        }
        points
    }
}

impl Turn for Slide {
    fn play(&self, board: &mut Board) -> () {
        assert!(self.offsets.len() == board.grid[self.x][self.y].len(),
                "Trying to move a different number of pieces than exist.");

        let cell = mem::replace(&mut board.grid[self.x][self.y], Cell::new());
        for (point, piece) in self.locations().iter().zip(cell.pieces.iter()) {
            let (x, y) = *point;
            board.grid[x][y].add_piece(*piece);
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

    fn play<T: Turn>(&mut self, turn: T) -> () {
        turn.play(self);
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
