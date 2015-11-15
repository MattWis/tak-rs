extern crate tak;

use tak::play;
use tak::Turn;
use tak::Board;
use tak::Player;

fn play_no_win(moves: Vec<&str>, game: &mut Board) -> () {
    for str in moves {
        play(&(str.parse::<Turn>().unwrap()), game).unwrap();
        assert_eq!(game.check_winner(), None);
    }
}

#[test]
fn basic_placement() {
    let mut game = Board::new(4);
    play_no_win(vec!["a1S1", "a2F2", "d3C2"], &mut game);
    assert_eq!(game.to_string(), "____________\n\
                                  |  |  |  |  \n\
                                  |  |  |  |C2\n\
                                  |F2|  |  |  \n\
                                  |S1|  |  |  \n");
}

#[test]
fn basic_movement() {
    let mut game = Board::new(4);
    play_no_win(vec!["a1S1", "a2F2", "a1U1", "a2R12"], &mut game);
    assert_eq!(game.to_string(), "____________\n\
                                  |  |  |  |  \n\
                                  |  |  |  |  \n\
                                  |  |F2|S1|  \n\
                                  |  |  |  |  \n");
}

#[test]
fn invalid_movement_onto_standing() {
    let mut game = Board::new(4);
    play_no_win(vec!["a1S1", "a2F2"], &mut game);
    match play(&("a2D1".parse::<Turn>().unwrap()), &mut game) {
        Ok(_) => panic!(""),
        Err(_) => return,
    }

}

#[test]
fn invalid_movement_onto_capstone() {
    let mut game = Board::new(4);
    play_no_win(vec!["a1C1", "a2F2"], &mut game);
    match play(&("a2D1".parse::<Turn>().unwrap()), &mut game) {
        Ok(_) => panic!(""),
        Err(_) => return,
    }
}

#[test]
fn squash() {
    let mut game = Board::new(4);
    play_no_win(vec!["a1S1", "a2C2", "a2D1"], &mut game);
    assert_eq!(game.to_string(), "____________________\n\
                                  |    |    |    |    \n\
                                  |    |    |    |    \n\
                                  |    |    |    |    \n\
                                  |F1C2|    |    |    \n");
}

#[test]
fn win_across() {
    let mut game = Board::new(4);
    play_no_win(vec!["a1F1", "a2C1", "a3F1"], &mut game);
    play(&("a4F1".parse::<Turn>().unwrap()), &mut game).unwrap();
    assert_eq!(game.check_winner(), Some(Player::One));
}

#[test]
fn almost() {
    play_no_win(vec!["a1F1", "b1F1", "c1F1"], &mut Board::new(4));
}

#[test]
fn almost2() {
    play_no_win(vec!["b1F1", "c1F1", "d1F1"], &mut Board::new(4));
}

#[test]
fn win_up() {
    let mut game = Board::new(4);
    play_no_win(vec!["a1F1", "b1F1", "c1F1"], &mut game);
    play(&("d1C1".parse::<Turn>().unwrap()), &mut game).unwrap();
    assert_eq!(game.check_winner(), Some(Player::One));
}

#[test]
fn cant_win_with_standing() {
    let mut game = Board::new(4);
    play_no_win(vec!["a1F1", "b1F1", "c1F1"], &mut game);
    play(&("d1S1".parse::<Turn>().unwrap()), &mut game).unwrap();
    assert_eq!(game.check_winner(), None);
}

#[test]
fn readme_game() {
    let mut game = Board::new(5);
    let m = vec!["a1F2", "e1F1", "d2F1", "c3F2", "d3F1", "d4F2", "d5F1", "c4F2",
                 "e2F1", "c3R1", "c3F1", "b4F2", "e4F1", "e3F2", "a4F1", "b3F2",
                 "d5D1", "b1F2", "c3L1", "b2F2", "a4R1", "c3F2", "e4D1", "c2F2",
                 "d2L1", "c5F2", "e3L12"];
    play_no_win(m, &mut game);
    play(&("d3L012".parse::<Turn>().unwrap()), &mut game).unwrap();
    assert_eq!(game.check_winner(), Some(Player::Two));
}

#[test]
fn example_game1() {
    let mut game = Board::new(5);
    let m = vec!["a1F2", "e5F1", "c3F1", "d3F2", "e3F1", "e2F2", "e4F1", "c2F2",
                 "b3F1", "a3F2", "d4F1", "b2F2", "a4F1", "a2F2", "d2S1", "d1C2",
                 "d2L1", "a5F2", "a4D1", "c2U12", "d4D1", "b2U1", "c2F1",
                 "a4F2", "a3D01"];
    play_no_win(m, &mut game);
    play(&("a2R01".parse::<Turn>().unwrap()), &mut game).unwrap();
    assert_eq!(game.check_winner(), Some(Player::Two));
}

#[test]
fn example_game2() {
    let mut game = Board::new(5);
    let m = vec!["a1F2", "e1F1", "c3F1", "c2F2", "a3F1", "d3F2", "b3F1", "e3F2",
                 "e2F1", "d2F2", "d4F1", "b1F2", "c3D1", "c3F2", "d1F1", "c1F2",
                 "d1L1", "d1F2", "e1L1", "b2F2", "b3D1", "d1L11", "a2S1"];
    play_no_win(m, &mut game);
    play(&("c1U0112".parse::<Turn>().unwrap()), &mut game).unwrap();
    assert_eq!(game.check_winner(), Some(Player::Two));
}

