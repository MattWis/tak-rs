use std::mem;

pub mod piece;
pub mod point;
pub mod turn;
pub mod board;

pub use turn::Turn;

pub fn play(turn: &Turn, board: &mut board::Board) -> () {
    match turn {
        &Turn::Placement { ref point, ref piece } => {
            board.at(point).place_piece(*piece);
        }
        &Turn::Slide { ref point, ref direction, ref offsets } => {
            assert!(offsets.len() == board.at(point).len(),
                    "Trying to move a different number of pieces than exist.");

            let cell = mem::replace(board.at(point), board::Square::new());
            let points = offsets.iter().map(|z| {
                direction.adjust(point, *z, board.size()).unwrap()
            }).collect::<Vec<_>>();

            for (point, piece) in points.iter().zip(cell.pieces.iter()) {
                board.at(&point).add_piece(*piece);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_placement() {
        let mut game = board::Board::new(4);
        play(&("a1S1".parse::<Turn>().unwrap()), &mut game);
        play(&("a2F2".parse::<Turn>().unwrap()), &mut game);
        play(&("d3C2".parse::<Turn>().unwrap()), &mut game);
        println!("{}", game);
        assert_eq!(game.to_string(), "____________\n\
                                      |  |  |  |  \n\
                                      |  |  |  |C2\n\
                                      |F2|  |  |  \n\
                                      |S1|  |  |  \n");
    }

    #[test]
    fn basic_movement() {
        let mut game = board::Board::new(4);
        play(&("a1S1".parse::<Turn>().unwrap()), &mut game);
        play(&("a2F2".parse::<Turn>().unwrap()), &mut game);
        play(&("a1U1".parse::<Turn>().unwrap()), &mut game);
        play(&("a2R12".parse::<Turn>().unwrap()), &mut game);
        assert_eq!(game.to_string(), "____________\n\
                                      |  |  |  |  \n\
                                      |  |  |  |  \n\
                                      |  |F2|S1|  \n\
                                      |  |  |  |  \n");
    }

    #[test]
    #[should_panic]
    fn invalid_movement_onto_standing() {
        let mut game = board::Board::new(4);
        play(&("a1S1".parse::<Turn>().unwrap()), &mut game);
        play(&("a2F2".parse::<Turn>().unwrap()), &mut game);
        play(&("a2D1".parse::<Turn>().unwrap()), &mut game);
    }

    #[test]
    #[should_panic]
    fn invalid_movement_onto_capstone() {
        let mut game = board::Board::new(4);
        play(&("a1C1".parse::<Turn>().unwrap()), &mut game);
        play(&("a2F2".parse::<Turn>().unwrap()), &mut game);
        play(&("a2D1".parse::<Turn>().unwrap()), &mut game);
    }

    #[test]
    fn squash() {
        let mut game = board::Board::new(4);
        play(&("a1S1".parse::<Turn>().unwrap()), &mut game);
        play(&("a2C2".parse::<Turn>().unwrap()), &mut game);
        play(&("a2D1".parse::<Turn>().unwrap()), &mut game);
        assert_eq!(game.to_string(), "____________________\n\
                                      |    |    |    |    \n\
                                      |    |    |    |    \n\
                                      |    |    |    |    \n\
                                      |F1C2|    |    |    \n");
    }

    #[test]
    fn win_across() {
        let mut game = board::Board::new(4);
        play(&("a1F1".parse::<Turn>().unwrap()), &mut game);
        play(&("a2C1".parse::<Turn>().unwrap()), &mut game);
        play(&("a3F1".parse::<Turn>().unwrap()), &mut game);
        play(&("a4F1".parse::<Turn>().unwrap()), &mut game);
        assert_eq!(game.check_winner(), Some(piece::Player::One));
    }

    #[test]
    fn almost() {
        let mut game = board::Board::new(4);
        play(&("a1F1".parse::<Turn>().unwrap()), &mut game);
        play(&("b1F1".parse::<Turn>().unwrap()), &mut game);
        play(&("c1F1".parse::<Turn>().unwrap()), &mut game);
        assert_eq!(game.check_winner(), None);
    }

    #[test]
    fn win_up() {
        let mut game = board::Board::new(4);
        play(&("a1F1".parse::<Turn>().unwrap()), &mut game);
        play(&("b1F1".parse::<Turn>().unwrap()), &mut game);
        play(&("c1F1".parse::<Turn>().unwrap()), &mut game);
        play(&("d1C1".parse::<Turn>().unwrap()), &mut game);
        assert_eq!(game.check_winner(), Some(piece::Player::One));
    }

    #[test]
    fn cant_win_with_standing() {
        let mut game = board::Board::new(4);
        play(&("a1F1".parse::<Turn>().unwrap()), &mut game);
        play(&("b1F1".parse::<Turn>().unwrap()), &mut game);
        play(&("c1F1".parse::<Turn>().unwrap()), &mut game);
        play(&("d1S1".parse::<Turn>().unwrap()), &mut game);
        assert_eq!(game.check_winner(), None);
    }
}

fn main () {
    let mut game = board::Board::new(4);
    play(&("a1S1".parse::<Turn>().unwrap()), &mut game);
    play(&("a2C2".parse::<Turn>().unwrap()), &mut game);
    play(&("a1U1".parse::<Turn>().unwrap()), &mut game);
    play(&("a2R12".parse::<Turn>().unwrap()), &mut game);
    println!("{}", game);
}
