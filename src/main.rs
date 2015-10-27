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
        assert!(self.pieces.len() == 0, "Cannot place stone on top of existing stone.");
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

struct Slide {
    direction: Direction,
    offsets: Vec<usize>,
}

impl Slide {
    fn locations(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut points = Vec::<(usize, usize)>::new();
        for offset in self.offsets.iter() {
            points.push(self.direction.adjust(x, y, *offset))
        }
        points
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

    fn place(&mut self, x: usize, y: usize, piece: Piece) -> () {
        self.grid[x][y].place_piece(piece);
    }

    fn slide(&mut self, x: usize, y: usize, movement: Slide) -> () {
        assert!(movement.offsets.len() == self.grid[x][y].len(),
                "Trying to move a different number of pieces than exist.");

        let cell = self.grid[x].remove(y);
        self.grid[x].insert(y, Cell::new());

        let points = movement.locations(x, y);
        for (point, piece) in points.iter().zip(cell.pieces.iter()) {
            let (new_x, new_y) = *point;
            self.grid[new_x][new_y].add_piece(*piece);
        }

    }
}

fn main() {
    let mut game = Board::new(4);
    game.place(0, 0, Piece { stone: Stone::Standing, owner: Player::One });
    game.place(0, 1, Piece { stone: Stone::Flat, owner: Player::Two });
    game.slide(0, 0, Slide { direction: Direction::Up, offsets: vec![1]});
    game.slide(0, 1, Slide { direction: Direction::Right, offsets: vec![1, 2]});
    println!("{:#?}", game);
}
