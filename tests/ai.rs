extern crate tak;

use tak::Game;
use tak::Turn;
use tak::Stone;
use tak::Point;
use tak::Player;
use tak::Ai;

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
