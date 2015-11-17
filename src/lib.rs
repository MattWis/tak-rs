use std::mem;

pub mod piece;
pub mod point;
pub mod turn;
pub mod board;

pub use turn::Turn;
pub use board::Board;
pub use piece::Player;

pub fn play(turn: &Turn, board: &mut Board) -> Result<(), String> {
    match turn {
        &Turn::Placement { ref point, ref piece } => {
            let square = try!(board.at_mut(point));
            try!(square.place_piece(*piece));
        }
        &Turn::Slide { ref point, ref direction, ref offsets } => {
            let cell = {
                let square = try!(board.at_mut(point));
                if offsets.len() != square.len() {
                    return Err("Trying to move a different number of pieces than exist".into());
                }

                mem::replace(square, board::Square::new())
            };
            let points = offsets.iter()
                                .map(|z| direction.adjust(&Some(*point), *z, board.size()))
                                .collect::<Vec<_>>();

            for (point, piece) in points.iter().zip(cell.pieces.iter()) {
                let p = match *point {
                    Some(x) => x,
                    None => return Err("".into()),
                };
                let square = try!(board.at_mut(&p));
                try!(square.add_piece(*piece));
            }
        }
    }
    Ok(())
}
