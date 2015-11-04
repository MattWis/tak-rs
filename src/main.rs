use std::mem;

pub mod piece;
pub mod point;
pub mod turn;
pub mod board;

fn play(turn: &turn::Turn, board: &mut board::Board) -> () {
    match turn {
        &turn::Turn::Placement { ref point, ref piece } => {
            board.at(point).place_piece(*piece);
        }
        &turn::Turn::Slide { ref point, ref direction, ref offsets } => {
            assert!(offsets.len() == board.at(point).len(),
                    "Trying to move a different number of pieces than exist.");

            let cell = mem::replace(board.at(point), board::Square::new());
            let points = offsets.iter().map(|z| direction.adjust(point, *z));
            for (point, piece) in points.zip(cell.pieces.iter()) {
                board.at(&point).add_piece(*piece);
            }
        }
    }
}

#[test]
fn basic_placement() {
    let mut game = board::Board::new(4);
    play(&("a1S1".parse::<turn::Turn>().unwrap()), &mut game);
    play(&("a2F2".parse::<turn::Turn>().unwrap()), &mut game);
    play(&("d3C2".parse::<turn::Turn>().unwrap()), &mut game);
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
    play(&("a1S1".parse::<turn::Turn>().unwrap()), &mut game);
    play(&("a2F2".parse::<turn::Turn>().unwrap()), &mut game);
    play(&("a1U1".parse::<turn::Turn>().unwrap()), &mut game);
    play(&("a2R12".parse::<turn::Turn>().unwrap()), &mut game);
    assert_eq!(game.to_string(), "____________\n\
                                  |  |  |  |  \n\
                                  |  |  |  |  \n\
                                  |  |F2|S1|  \n\
                                  |  |  |  |  \n");
}

#[test]
#[should_panic]
fn invalid_movement() {
    let mut game = board::Board::new(4);
    play(&("a1S1".parse::<turn::Turn>().unwrap()), &mut game);
    play(&("a2F2".parse::<turn::Turn>().unwrap()), &mut game);
    play(&("a2D1".parse::<turn::Turn>().unwrap()), &mut game);
}

#[test]
fn squash() {
    let mut game = board::Board::new(4);
    play(&("a1S1".parse::<turn::Turn>().unwrap()), &mut game);
    play(&("a2C2".parse::<turn::Turn>().unwrap()), &mut game);
    play(&("a2D1".parse::<turn::Turn>().unwrap()), &mut game);
    assert_eq!(game.to_string(), "____________________\n\
                                  |    |    |    |    \n\
                                  |    |    |    |    \n\
                                  |    |    |    |    \n\
                                  |F1C2|    |    |    \n");
}

fn main () {
    let mut game = board::Board::new(4);
    play(&("a1S1".parse::<turn::Turn>().unwrap()), &mut game);
    play(&("a2C2".parse::<turn::Turn>().unwrap()), &mut game);
    play(&("a1U1".parse::<turn::Turn>().unwrap()), &mut game);
    play(&("a2R12".parse::<turn::Turn>().unwrap()), &mut game);
    println!("{}", game);
}
