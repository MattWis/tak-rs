extern crate tak;

use tak::Game;
use tak::Player;

fn play_no_win(moves: Vec<&str>, game: &mut Game) -> () {
    let mut player = Player::One;
    for str in moves {
        println!("{}", str);
        assert_eq!(game.play(str, player).unwrap(), None);
        player = player.other();
    }
}

#[test]
fn basic_placement() {
    let mut game = Game::new(5);
    play_no_win(vec!["a2F2", "d1F1", "a1S1", "d3C2"], &mut game);
    assert_eq!(game.to_string(),
               "_______________\n\
                |  |  |  |  |  \n\
                |  |  |  |  |  \n\
                |  |  |  |C2|  \n\
                |F2|  |  |  |  \n\
                |S1|  |  |F1|  \n");
}

#[test]
fn basic_movement() {
    let mut game = Game::new(4);
    play_no_win(vec!["a2F2", "a1F1", "a1+1", "b1F2", "2a2>11"], &mut game);
    assert_eq!(game.to_string(),
               "____________\n\
                |  |  |  |  \n\
                |  |  |  |  \n\
                |  |F2|F1|  \n\
                |  |F2|  |  \n");
}

#[test]
fn must_own_pile_to_move() {
    let mut game = Game::new(5);
    play_no_win(vec!["a1F2", "a2F1", "a2-1"], &mut game);
    match game.play("1a1>1", Player::Two) {
        Ok(_) => panic!(""),
        Err(_) => return,
    }
}


#[test]
fn invalid_movement_onto_standing() {
    let mut game = Game::new(5);
    play_no_win(vec!["a2F2", "a4F1", "a1S1"], &mut game);
    match game.play("a2-1", Player::Two) {
        Ok(_) => panic!(""),
        Err(_) => return,
    }
}

#[test]
fn starting_order() {
    match Game::new(4).play("a2F1", Player::One) {
        Ok(_) => panic!(""),
        Err(_) => return,
    }
}

#[test]
fn starting_stone() {
    match Game::new(4).play("a2S2", Player::One) {
        Ok(_) => panic!(""),
        Err(_) => return,
    }
}

#[test]
fn move_offstage() {
    let mut game = Game::new(5);
    play_no_win(vec!["a1F2", "a2F1"], &mut game);
    match game.play("a2<1", Player::One) {
        Ok(_) => panic!(""),
        Err(_) => return,
    }
}

#[test]
fn under_carry_limit() {
    let mut game = Game::new(5);
    play_no_win(vec!["a1F2", "a2F1", "a2-1", "b1F2", "a2F1", "b1<1", "a2-1",
                     "b1F2", "a2F1", "b1<1", "a2-1", "b1F2", "5a1>5"],
                &mut game);
}

#[test]
fn carry_limit() {
    let mut game = Game::new(5);
    play_no_win(vec!["a1F2", "a2F1", "a2-1", "b1F2", "a2F1", "b1<1", "a2-1",
                     "b1F2", "a2F1", "b1<1", "a2-1", "b1F2"], &mut game);
    match game.play("6a1>6", Player::One) {
        Ok(_) => panic!(""),
        Err(_) => return,
    }
}

#[test]
fn movement_amount() {
    let mut game = Game::new(5);
    play_no_win(vec!["a1F2", "a2F1", "a2-1", "b1F2", "a2F1", "b1<1", "a2-1",
                     "b1F2"], &mut game);
    match game.play("4a1>022", Player::One) {
        Ok(_) => panic!(""),
        Err(_) => return,
    }
}

#[test]
fn invalid_movement_onto_capstone() {
    let mut game = Game::new(5);
    play_no_win(vec!["a2F2", "c3F1", "a1C1"], &mut game);
    match game.play("a2-1", Player::Two) {
        Ok(_) => panic!(""),
        Err(_) => return,
    }
}

#[test]
fn squash() {
    let mut game = Game::new(5);
    play_no_win(vec!["b2F2", "b1F1", "a1S1", "a2C2", "b3F1", "a2-1"],
                &mut game);
    assert_eq!(game.to_string(),
               "_________________________\n\
                |    |    |    |    |    \n\
                |    |    |    |    |    \n\
                |    |F1  |    |    |    \n\
                |    |F2  |    |    |    \n\
                |F1C2|F1  |    |    |    \n");
}

#[test]
fn win_across() {
    let mut game = Game::new(4);
    let m = vec!["b1F2", "a1F1", "a2F1", "b2F2", "a3F1", "b3F2"];
    play_no_win(m, &mut game);
    assert_eq!(game.play("a4F1", Player::One).unwrap(), Some(Player::One));
}

#[test]
fn almost() {
    play_no_win(vec!["a2F2", "a1F1", "b1F1", "b2F2", "c1F1"],
                &mut Game::new(4));
}

#[test]
fn almost2() {
    play_no_win(vec!["a2F2", "b1F1", "c1F1", "b2F2", "d1F1"],
                &mut Game::new(4));
}

#[test]
fn win_up() {
    let mut game = Game::new(4);
    let m = vec!["a2F2", "a1F1", "b1F1", "b2F2", "c1F1", "c2F2"];
    play_no_win(m, &mut game);
    assert_eq!(game.play("d1F1", Player::One).unwrap(), Some(Player::One));
}

#[test]
fn cant_win_with_standing() {
    let mut game = Game::new(4);
    let m = vec!["a2F2", "a1F1", "b1F1", "b2F2", "c1F1", "c2F2"];
    play_no_win(m, &mut game);
    assert_eq!(game.play("d1S1", Player::One).unwrap(), None);
}

#[test]
fn all_pieces() {
    let mut game = Game::new(4);
    let m = vec!["a2F2", "a1F1", "a3F1", "a4F2", "b2F1", "b1F2", "b4F1", "b3F2", "c1F1", "c2F2",
                 "c3F1", "c4F2", "d2F1", "d1F2", "d4F1", "d1<1", "d1F1", "b1<1", "b1F1", "a2>1",
                 "a2F1", "c2>1", "c2F1", "b3<1", "b3F1", "a4>1", "a4F1", "c4<1"];
    play_no_win(m, &mut game);
    assert_eq!(game.play("c4F1", Player::One).unwrap(), Some(Player::One));
}

#[test]
fn all_pieces_with_cap() {
    let mut game = Game::new(5);
    let m = vec!["e1F2", "a1F1", "a2C1", "e1+1", "a3F1", "e2-1", "a4F1", "e1+1", "b1F1", "e2-1",
                 "b2F1", "e1+1", "b3F1", "e2-1", "b4F1", "e1+1", "c1F1", "e2-1", "c2F1", "e1+1",
                 "c3F1", "e2-1", "c4F1", "e1+1", "d1F1", "e2-1", "d2F1", "e1+1", "d3F1", "e2-1",
                 "d4F1", "e1+1", "d4<1", "e2<1", "d3<1", "2d2-2", "d4F1", "3d1<3", "d3F1",
                 "c1+1", "d2F1", "2c2<2", "d1F1", "e1F2"];
    play_no_win(m, &mut game);
    assert_eq!(game.play("e5F1", Player::One).unwrap(), Some(Player::One));
}

#[test]
fn full_board() {
    let mut game = Game::new(4);
    let m = vec!["a2F2", "a1F1", "a3F1", "a4F2", "b2F1", "b1F2", "b4F1", "b3F2", "c1F1", "c2F2",
                 "c3F1", "c4F2", "d2F1", "d1F2", "d4F1"];
    play_no_win(m, &mut game);
    assert_eq!(game.play("d3F2", Player::Two).unwrap(), Some(Player::Two));
}

#[test]
fn cannot_play_too_many_capstones() {
    let mut game = Game::new(5);
    let m = vec!["e1F2", "a1F1", "c3C1", "c2F2"];
    play_no_win(m, &mut game);
    match game.play("c4C1", Player::One) {
        Ok(_) => panic!(""),
        Err(_) => return,
    }
}

#[test]
fn cannot_play_too_many_flats() {
    let mut game = Game::new(5);
    let m = vec!["e1F2", "a1F1", "a2F1", "e1+1", "a3F1", "e2-1", "a4F1", "e1+1", "b1F1", "e2-1",
                 "b2F1", "e1+1", "b3F1", "e2-1", "b4F1", "e1+1", "c1F1", "e2-1", "c2F1", "e1+1",
                 "c3F1", "e2-1", "c4F1", "e1+1", "d1F1", "e2-1", "d2F1", "e1+1", "d3F1", "e2-1",
                 "d4F1", "e1+1", "d4<1", "e2<1", "d3<1", "2d2-2", "d4F1", "3d1<3", "d3F1",
                 "c1+1", "d2F1", "2c2<2", "d1F1", "e1F2"];
    play_no_win(m, &mut game);
    match game.play("e5F1", Player::One) {
        Ok(_) => panic!(""),
        Err(_) => return,
    }
}

#[test]
fn convoluted_road_win() {
    let mut game = Game::new(6);
    let m = vec!["a2F2", "a1F1", "b1F1", "b2F2", "c1F1", "c2F2", "d1F1", "e1F2", "d2F1", "e2F2",
                 "d3F1", "e3F2", "c3F1", "e4F2", "b3F1", "d4F2", "b4F1", "c4F2", "b5F1", "a3F2",
                 "c5F1", "a4F2", "d5F1", "a5F2", "e5F1", "a6F2"];
    play_no_win(m, &mut game);
    assert_eq!(game.play("f5F1", Player::One).unwrap(), Some(Player::One));
}

#[test]
fn example1() {
    let mut game = Game::new(5);
    let m = vec!["a1F2", "e1F1", "c3F1", "a3F2", "e3F1", "a4F2", "a2F1", "a5F2", "b4F1", "d3F2",
                 "e2F1", "e4F2", "d2F1", "b2F2", "b4<1", "a3+1", "a3F1", "3a4-12", "a4F1", "b3F2",
                 "b4F1", "b3+1", "a4+1", "a4F2", "b3F1", "2b4-2", "d4F1", "b5F2", "2a5-11",
                 "3b3<3", "a5F1"];
    play_no_win(m, &mut game);
    assert_eq!(game.play("a4+1", Player::Two).unwrap(), Some(Player::Two));
}
