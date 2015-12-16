extern crate tak;

use tak::Game;
use tak::Player;

fn play_no_win(moves: Vec<&str>, game: &mut Game) -> () {
    let mut turn = 0;
    for str in moves {
        println!("{}", str);
        if turn >= 2 {
            let player = if turn % 2 == 0 {
                Player::One
            } else {
                Player::Two
            };
            assert_eq!(game.play(str, player, Some(player)).unwrap(), None);
        } else {
            let player = if turn % 2 == 0 {
                Player::One
            } else {
                Player::Two
            };
            assert_eq!(game.play(str, player, Some(player.other())).unwrap(), None);
        }
        turn += 1;
    }
}

#[test]
fn basic_placement() {
    let mut game = Game::new(5);
    play_no_win(vec!["Fa2", "d1", "Sa1", "Cd3"], &mut game);
    assert_eq!(game.to_string(),
               "_______________\n\
                |  |  |  |  |  \n\
                |  |  |  |  |  \n\
                |  |  |  |C2|  \n\
                |F2|  |  |  |  \n\
                |S1|  |  |F1|  \n\
                P1: 2/20 Flatstones\n\
                P1: 0/1 Capstones\n\
                P2: 1/20 Flatstones\n\
                P2: 1/1 Capstones\n");
}

#[test]
fn basic_movement() {
    let mut game = Game::new(4);
    play_no_win(vec!["a2", "a1", "a1+", "b1", "2a2>11"], &mut game);
    assert_eq!(game.to_string(),
               "____________\n\
                |  |  |  |  \n\
                |  |  |  |  \n\
                |  |F2|F1|  \n\
                |  |F2|  |  \n\
                P1: 1/15 Flatstones\n\
                P1: 0/0 Capstones\n\
                P2: 2/15 Flatstones\n\
                P2: 0/0 Capstones\n");
}

#[test]
fn break_parser() {
    let mut game = Game::new(4);
    match game.play("0a1", Player::One, None) {
        Ok(_) => panic!(""),
        Err(_) => return,
    }
}

#[test]
fn must_own_pile_to_move() {
    let mut game = Game::new(5);
    play_no_win(vec!["a1", "a2", "a2-"], &mut game);
    match game.play("1a1>1", Player::Two, None) {
        Ok(_) => panic!(""),
        Err(_) => return,
    }
}


#[test]
fn invalid_movement_onto_standing() {
    let mut game = Game::new(5);
    play_no_win(vec!["a2", "a4", "Sa1"], &mut game);
    match game.play("a2-", Player::Two, None) {
        Ok(_) => panic!(""),
        Err(_) => return,
    }
}

#[test]
fn starting_order() {
    match Game::new(4).play("a2", Player::One, Some(Player::One)) {
        Ok(_) => panic!(""),
        Err(_) => return,
    }
}

#[test]
fn starting_stone() {
    match Game::new(4).play("Sa2", Player::One, Some(Player::Two)) {
        Ok(_) => panic!(""),
        Err(_) => return,
    }
}

#[test]
fn move_offstage() {
    let mut game = Game::new(5);
    play_no_win(vec!["a1", "a2"], &mut game);
    match game.play("a2<", Player::One, None) {
        Ok(_) => panic!(""),
        Err(_) => return,
    }
}

#[test]
fn under_carry_limit() {
    let mut game = Game::new(5);
    play_no_win(vec!["a1", "a2", "a2-", "b1", "a2", "b1<", "a2-",
                     "b1", "a2", "b1<", "a2-", "b1", "5a1>5"],
                &mut game);
}

#[test]
fn carry_limit() {
    let mut game = Game::new(5);
    play_no_win(vec!["a1", "a2", "a2-", "b1", "a2", "b1<", "a2-",
                     "b1", "a2", "b1<", "a2-", "b1"], &mut game);
    match game.play("6a1>6", Player::One, None) {
        Ok(_) => panic!(""),
        Err(_) => return,
    }
}

#[test]
fn movement_amount() {
    let mut game = Game::new(5);
    play_no_win(vec!["a1", "a2", "a2-", "b1", "a2", "b1<", "a2-",
                     "b1"], &mut game);
    match game.play("4a1>022", Player::One, None) {
        Ok(_) => panic!(""),
        Err(_) => return,
    }
}

#[test]
fn invalid_movement_onto_capstone() {
    let mut game = Game::new(5);
    play_no_win(vec!["a2", "c3", "Ca1"], &mut game);
    match game.play("a2-", Player::Two, None) {
        Ok(_) => panic!(""),
        Err(_) => return,
    }
}

#[test]
fn squash() {
    let mut game = Game::new(5);
    play_no_win(vec!["b2", "b1", "Sa1", "Ca2", "b3", "a2-"],
                &mut game);
    assert_eq!(game.to_string(),
               "_________________________\n\
                |    |    |    |    |    \n\
                |    |    |    |    |    \n\
                |    |F1  |    |    |    \n\
                |    |F2  |    |    |    \n\
                |F1C2|F1  |    |    |    \n\
                P1: 3/20 Flatstones\n\
                P1: 0/1 Capstones\n\
                P2: 1/20 Flatstones\n\
                P2: 1/1 Capstones\n");
}

#[test]
fn win_across() {
    let mut game = Game::new(4);
    let m = vec!["b1", "a1", "a2", "b2", "a3", "b3"];
    play_no_win(m, &mut game);
    assert_eq!(game.play("a4", Player::One, Some(Player::One)).unwrap(), Some(Player::One));
}

#[test]
fn almost() {
    play_no_win(vec!["a2", "a1", "b1", "b2", "c1"],
                &mut Game::new(4));
}

#[test]
fn almost2() {
    play_no_win(vec!["a2", "b1", "c1", "b2", "d1"],
                &mut Game::new(4));
}

#[test]
fn win_up() {
    let mut game = Game::new(4);
    let m = vec!["a2", "a1", "b1", "b2", "c1", "c2"];
    play_no_win(m, &mut game);
    assert_eq!(game.play("d1", Player::One, Some(Player::One)).unwrap(), Some(Player::One));
}

#[test]
fn cant_win_with_standing() {
    let mut game = Game::new(4);
    let m = vec!["a2", "a1", "b1", "b2", "c1", "c2"];
    play_no_win(m, &mut game);
    assert_eq!(game.play("Sd1", Player::One, Some(Player::One)).unwrap(), None);
}

#[test]
fn all_pieces() {
    let mut game = Game::new(4);
    let m = vec!["a2", "a1", "a3", "a4", "b2", "b1", "b4", "b3", "c1", "c2",
                 "c3", "c4", "d2", "d1", "d4", "d1<", "d1", "b1<", "b1", "a2>",
                 "a2", "c2>", "c2", "b3<", "b3", "a4>", "a4", "c4<"];
    play_no_win(m, &mut game);
    assert_eq!(game.play("c4", Player::One, Some(Player::One)).unwrap(), Some(Player::One));
}

#[test]
fn all_pieces_with_cap() {
    let mut game = Game::new(5);
    let m = vec!["e1", "a1", "Ca2", "e1+", "a3", "e2-", "a4", "e1+", "b1", "e2-",
                 "b2", "e1+", "b3", "e2-", "b4", "e1+", "c1", "e2-", "c2", "e1+",
                 "c3", "e2-", "c4", "e1+", "d1", "e2-", "d2", "e1+", "d3", "e2-",
                 "d4", "e1+", "d4<", "e2<", "d3<", "2d2-2", "d4", "3d1<3", "d3",
                 "c1+", "d2", "2c2<2", "d1", "e1"];
    play_no_win(m, &mut game);
    assert_eq!(game.play("e5", Player::One, Some(Player::One)).unwrap(), Some(Player::One));
}

#[test]
fn full_board() {
    let mut game = Game::new(4);
    let m = vec!["a2", "a1", "a3", "a4", "b2", "b1", "b4", "b3", "c1", "c2",
                 "c3", "c4", "d2", "d1", "d4"];
    play_no_win(m, &mut game);
    assert_eq!(game.play("d3", Player::Two, Some(Player::Two)).unwrap(), Some(Player::Two));
}

#[test]
fn cannot_play_too_many_capstones() {
    let mut game = Game::new(5);
    let m = vec!["e1", "a1", "Cc3", "c2"];
    play_no_win(m, &mut game);
    match game.play("Cc4", Player::One, Some(Player::One)) {
        Ok(_) => panic!(""),
        Err(_) => return,
    }
}

#[test]
fn cannot_play_too_many_flats() {
    let mut game = Game::new(5);
    let m = vec!["e1", "a1", "a2", "e1+", "a3", "e2-", "a4", "e1+", "b1", "e2-",
                 "b2", "e1+", "b3", "e2-", "b4", "e1+", "c1", "e2-", "c2", "e1+",
                 "c3", "e2-", "c4", "e1+", "d1", "e2-", "d2", "e1+", "d3", "e2-",
                 "d4", "e1+", "d4<", "e2<", "d3<", "2d2-2", "d4", "3d1<3", "d3",
                 "c1+", "d2", "2c2<2", "d1", "e1"];
    play_no_win(m, &mut game);
    match game.play("e5", Player::One, Some(Player::One)) {
        Ok(_) => panic!(""),
        Err(_) => return,
    }
}

#[test]
fn convoluted_road_win() {
    let mut game = Game::new(6);
    let m = vec!["a2", "a1", "b1", "b2", "c1", "c2", "d1", "e1", "d2", "e2",
                 "d3", "e3", "c3", "e4", "b3", "d4", "b4", "c4", "b5", "a3",
                 "c5", "a4", "d5", "a5", "e5", "a6"];
    play_no_win(m, &mut game);
    assert_eq!(game.play("f5", Player::One, Some(Player::One)).unwrap(), Some(Player::One));
}

#[test]
fn example1() {
    let mut game = Game::new(5);
    let m = vec!["a1", "e1", "c3", "a3", "e3", "a4", "a2", "a5", "b4", "d3",
                 "e2", "e4", "d2", "b2", "b4<", "a3+", "a3", "3a4-12", "a4", "b3",
                 "b4", "b3+", "a4+", "a4", "b3", "2b4-2", "d4", "b5", "2a5-11",
                 "3b3<3", "a5"];
    play_no_win(m, &mut game);
    assert_eq!(game.play("a4+", Player::Two, None).unwrap(), Some(Player::Two));
}
