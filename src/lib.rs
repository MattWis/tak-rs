use std::mem;

pub mod piece;
pub mod point;
pub mod turn;
pub mod board;

pub use turn::Turn;
pub use board::Board;
pub use piece::Player;

pub fn play(turn: &Turn, board: &mut board::Board) -> Result<(), String> {
    match turn {
        &Turn::Placement { ref point, ref piece } => {
            try!(board.at(point).place_piece(*piece));
        }
        &Turn::Slide { ref point, ref direction, ref offsets } => {
            assert!(offsets.len() == board.at(point).len(),
                    "Trying to move a different number of pieces than exist.");

            let cell = mem::replace(board.at(point), board::Square::new());
            let points = offsets.iter().map(|z| {
                direction.adjust(&Some(*point), *z, board.size())
            }).collect::<Vec<_>>();

            for (point, piece) in points.iter().zip(cell.pieces.iter()) {
                let p = match *point {
                    Some(x) => x,
                    None => return Err("".into()),
                };
                try!(board.at(&p).add_piece(*piece));
            }
        }
    }
    Ok(())
}
