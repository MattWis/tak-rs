use board::Board;
use turn::Turn;
use turn::Turn::Place;
use turn::Turn::Slide;
use piece::Player;
use piece::Stone;
use point::Point;

pub struct Ai {
    player: Player,
}

impl Ai {
    pub fn new(player: Player) -> Ai {
        Ai { player: player }
    }

    pub fn next_move(&self, turn: usize, board: &Board) -> Turn {
        if turn < 2 {
            if board.at(&Point::new(0, 0)).unwrap().pieces.is_empty() {
                Place { point: Point::new(0, 0), stone: Stone::Flat }
            } else {
                Place { point: Point::new(0, board.size() - 1), stone: Stone::Flat }
            }
        } else {
            Place { point: Point::new(0, 0), stone: Stone::Flat }
        }
    }
}
