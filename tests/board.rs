extern crate tak;

use tak::Board;
use tak::NaiveBoard;
use tak::Stone;
use tak::Point;
use tak::Piece;
use tak::Player;

#[test]
fn empty() {
    "x,x,x,x,x/x,x,x,x,x/x,x,x,x,x/x,x,x,x,x/x,x,x,x,x".parse::<NaiveBoard>().unwrap();
}

#[test]
fn empty_condensed() {
    "x5/x5/x5/x5/x5".parse::<NaiveBoard>().unwrap();
}

#[test]
fn fail_condensed() {
    match "x5/x5/x5/x5".parse::<NaiveBoard>() {
        Ok(_) => panic!(""),
        Err(_) => return,
    }
}

#[test]
fn r_tak_example() {
    "x5/x2,121,x2/x5/x,2C,x3/x5".parse::<NaiveBoard>().unwrap();
}

#[test]
fn single() {
    let g = "1,x,x,x,x/x,x,x,x,x/x,x,x,x,x/x,x,x,x,x/x,x,x,x,x".parse::<NaiveBoard>().unwrap();
    assert_eq!(g.at(&Point{ x: 0, y: 4}).unwrap().pieces[0], Piece::new(Stone::Flat, Player::One));
}

#[test]
fn stack() {
    let g = "122,x,x,x,x/x,x,x,x,x/x,x,x,x,x/x,x,x,x,x/x,x,x,x,x".parse::<NaiveBoard>().unwrap();
    assert_eq!(g.at(&Point{ x: 0, y: 4}).unwrap().pieces[0], Piece::new(Stone::Flat, Player::One));
    assert_eq!(g.at(&Point{ x: 0, y: 4}).unwrap().pieces[1], Piece::new(Stone::Flat, Player::Two));
    assert_eq!(g.at(&Point{ x: 0, y: 4}).unwrap().pieces[2], Piece::new(Stone::Flat, Player::Two));
}

#[test]
fn fail_stack() {
    match "x5/x4,1SS2/x5/x5/x5".parse::<NaiveBoard>() {
        Ok(_) => panic!(""),
        Err(_) => return,
    }
}
