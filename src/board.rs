use std::iter;
use std::fmt;

use piece;
use point;

#[derive(Clone, Debug)]
pub struct Square {
    pub pieces: Vec<piece::Piece>,
}

impl Square {
    pub fn new() -> Square {
        Square { pieces: vec![] }
    }

    pub fn len(&self) -> usize {
        self.pieces.len()
    }

    pub fn add_piece(&mut self, piece: piece::Piece) -> () {
        match self.pieces.last_mut() {
            Some(base) => piece.move_onto(base),
            None => {}
        }
        self.pieces.push(piece);
    }

    pub fn place_piece(&mut self, piece: piece::Piece) -> () {
        assert!(self.len() == 0,
                "Cannot place stone on top of existing stone.");
        self.pieces.push(piece);
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
}

#[derive(Debug)]
pub struct Board {
    grid: Vec<Vec<Square>>,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut full = String::new();
        let max = self.grid.iter()
                           .map(|row| row.iter().map(|cell| cell.len()).max())
                           .max().unwrap().unwrap();
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

    pub fn at(&mut self, point: &point::Point) -> &mut Square {
        &mut self.grid[point.y][point.x]
    }
}
