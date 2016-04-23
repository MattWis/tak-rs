extern crate tak;

use tak::Game;
use tak::Board;
use tak::NaiveBoard;
use tak::Turn;
use tak::Piece;
use tak::Stone;
use tak::Point;
use tak::Player;
use tak::Ai;

// First two turns, behavior is hard-coded
#[test]
fn goes_in_corner() {
    let game = Game::new(5);
    let ai = Ai::new(Player::One);
    assert_eq!(game.predict(ai), Turn::Place { point: Point::new(0, 0), stone: Stone::Flat })
}

#[test]
fn goes_in_other_corner() {
    let mut game = Game::new(5);
    let ai = Ai::new(Player::Two);
    game.play("a1", Player::One, Some(Player::Two)).unwrap();
    assert_eq!(game.predict(ai), Turn::Place { point: Point::new(0, 4), stone: Stone::Flat })
}

#[test]
fn moves_on_empty_board() {
    let ai = Ai::new(Player::One);
    assert_eq!(75, ai.possible_moves(&NaiveBoard::new(5)).len());
}

#[test]
fn no_capstones() {
    let ai = Ai::new(Player::One);
    assert_eq!(32, ai.possible_moves(&NaiveBoard::new(4)).len());
}

#[test]
fn basic_tower() {
    let ai = Ai::new(Player::One);
    let mut board = NaiveBoard::new(4);
    board.place_piece(&Point::new(1,1), Piece::new(Stone::Flat, Player::One)).unwrap();
    assert_eq!(34, ai.possible_moves(&board).len())
}

#[test]
fn unowned_tower() {
    let ai = Ai::new(Player::Two);
    let mut board = NaiveBoard::new(4);
    board.place_piece(&Point::new(1,1), Piece::new(Stone::Flat, Player::One)).unwrap();
    assert_eq!(30, ai.possible_moves(&board).len())
}

#[test]
fn blocked_tower() {
    let ai = Ai::new(Player::One);
    let mut board = NaiveBoard::new(4);
    board.place_piece(&Point::new(1,1), Piece::new(Stone::Flat, Player::One)).unwrap();
    board.place_piece(&Point::new(1,0), Piece::new(Stone::Standing, Player::Two)).unwrap();
    assert_eq!(31, ai.possible_moves(&board).len())
}

//#[test]
//fn capstone_tower() {
    //let ai = Ai::new(Player::One);
    //let mut board = NaiveBoard::new(4);
    //board.place_piece(&Point::new(1,1), Piece::new(Stone::Flat, Player::One)).unwrap();
    //board.at_mut(&Point::new(1,1)).unwrap().add_piece(Piece::new(Stone::Capstone, Player::One)).unwrap();
    //board.place_piece(&Point::new(1,0), Piece::new(Stone::Standing, Player::Two)).unwrap();
    //assert_eq!(28 + 3 + 3 + 2 + 1, ai.possible_moves(&board).len())
//}
