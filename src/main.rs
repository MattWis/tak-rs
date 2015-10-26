#[derive(Clone, Debug)]
enum PieceType {
    Flat,
    Standing,
    Capstone,
}

#[derive(Clone, Debug)]
enum PieceOwner {
    Player1,
    Player2,
}

#[derive(Clone, Debug)]
struct Piece {
    stone: PieceType,
    owner: PieceOwner,
}

#[derive(Clone, Debug)]
struct Cell {
    pieces: Vec<Piece>,
}

impl Cell {
    pub fn new() -> Cell {
        Cell { pieces: vec![] }
    }
}

#[derive(Debug)]
struct Board {
    grid: Vec<Vec<Cell>>,
}

impl Board {
    pub fn new(board_size: usize) -> Board {
        Board { grid: vec![vec![Cell::new(); board_size]; board_size] }
    }
}

fn main() {
    let game = Board::new(4);
    println!("{:?}", game);
}
