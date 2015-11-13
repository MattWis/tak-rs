extern crate tak;

use tak::play;
use tak::Turn;
use tak::Board;
use tak::Player;

#[test]
fn basic_placement() {
    let mut game = Board::new(4);
    play(&("a1S1".parse::<Turn>().unwrap()), &mut game).unwrap();
    play(&("a2F2".parse::<Turn>().unwrap()), &mut game).unwrap();
    play(&("d3C2".parse::<Turn>().unwrap()), &mut game).unwrap();
    println!("{}", game);
    assert_eq!(game.to_string(), "____________\n\
                                  |  |  |  |  \n\
                                  |  |  |  |C2\n\
                                  |F2|  |  |  \n\
                                  |S1|  |  |  \n");
}

#[test]
fn basic_movement() {
    let mut game = Board::new(4);
    play(&("a1S1".parse::<Turn>().unwrap()), &mut game).unwrap();
    play(&("a2F2".parse::<Turn>().unwrap()), &mut game).unwrap();
    play(&("a1U1".parse::<Turn>().unwrap()), &mut game).unwrap();
    play(&("a2R12".parse::<Turn>().unwrap()), &mut game).unwrap();
    assert_eq!(game.to_string(), "____________\n\
                                  |  |  |  |  \n\
                                  |  |  |  |  \n\
                                  |  |F2|S1|  \n\
                                  |  |  |  |  \n");
}

#[test]
fn invalid_movement_onto_standing() {
    let mut game = Board::new(4);
    play(&("a1S1".parse::<Turn>().unwrap()), &mut game).unwrap();
    play(&("a2F2".parse::<Turn>().unwrap()), &mut game).unwrap();
    match play(&("a2D1".parse::<Turn>().unwrap()), &mut game) {
        Ok(_) => panic!(""),
        Err(_) => return,
    }

}

#[test]
fn invalid_movement_onto_capstone() {
    let mut game = Board::new(4);
    play(&("a1C1".parse::<Turn>().unwrap()), &mut game).unwrap();
    play(&("a2F2".parse::<Turn>().unwrap()), &mut game).unwrap();
    match play(&("a2D1".parse::<Turn>().unwrap()), &mut game) {
        Ok(_) => panic!(""),
        Err(_) => return,
    }
}

#[test]
fn squash() {
    let mut game = Board::new(4);
    play(&("a1S1".parse::<Turn>().unwrap()), &mut game).unwrap();
    play(&("a2C2".parse::<Turn>().unwrap()), &mut game).unwrap();
    play(&("a2D1".parse::<Turn>().unwrap()), &mut game).unwrap();
    assert_eq!(game.to_string(), "____________________\n\
                                  |    |    |    |    \n\
                                  |    |    |    |    \n\
                                  |    |    |    |    \n\
                                  |F1C2|    |    |    \n");
}

#[test]
fn win_across() {
    let mut game = Board::new(4);
    play(&("a1F1".parse::<Turn>().unwrap()), &mut game).unwrap();
    play(&("a2C1".parse::<Turn>().unwrap()), &mut game).unwrap();
    play(&("a3F1".parse::<Turn>().unwrap()), &mut game).unwrap();
    play(&("a4F1".parse::<Turn>().unwrap()), &mut game).unwrap();
    assert_eq!(game.check_winner(), Some(Player::One));
}

#[test]
fn almost() {
    let mut game = Board::new(4);
    play(&("a1F1".parse::<Turn>().unwrap()), &mut game).unwrap();
    play(&("b1F1".parse::<Turn>().unwrap()), &mut game).unwrap();
    play(&("c1F1".parse::<Turn>().unwrap()), &mut game).unwrap();
    assert_eq!(game.check_winner(), None);
}

#[test]
fn win_up() {
    let mut game = Board::new(4);
    play(&("a1F1".parse::<Turn>().unwrap()), &mut game).unwrap();
    play(&("b1F1".parse::<Turn>().unwrap()), &mut game).unwrap();
    play(&("c1F1".parse::<Turn>().unwrap()), &mut game).unwrap();
    play(&("d1C1".parse::<Turn>().unwrap()), &mut game).unwrap();
    assert_eq!(game.check_winner(), Some(Player::One));
}

#[test]
fn cant_win_with_standing() {
    let mut game = Board::new(4);
    play(&("a1F1".parse::<Turn>().unwrap()), &mut game).unwrap();
    play(&("b1F1".parse::<Turn>().unwrap()), &mut game).unwrap();
    play(&("c1F1".parse::<Turn>().unwrap()), &mut game).unwrap();
    play(&("d1S1".parse::<Turn>().unwrap()), &mut game).unwrap();
    assert_eq!(game.check_winner(), None);
}
