use std::collections::BTreeSet;


use board::Board;
use board_naive::NaiveBoard;
use turn::Turn;
use turn::Turn::Place;
use turn::Turn::Slide;
use turn::Direction;
use piece::Piece;
use piece::Player;
use piece::Stone;
use point::Point;

pub struct Ai {
    player: Player,
}

impl Ai {
    pub fn new(player: Player) -> Ai {
        Ai { player: player }
    }

    pub fn next_move<T: Board>(&self, turn: usize, board: &T) -> Turn {
        if turn < 2 {
            if board.at(&Point::new(0, 0)).unwrap().count() == 0 {
                Place { point: Point::new(0, 0), stone: Stone::Flat }
            } else {
                Place { point: Point::new(0, board.size() - 1), stone: Stone::Flat }
            }
        } else {
            Place { point: Point::new(0, 0), stone: Stone::Flat }
        }
    }

    pub fn possible_moves<T: Board>(&self, board: &T) -> Vec<Turn> {
        let mut moves = vec![];
        for x in 0..board.size() {
            for y in 0..board.size() {
                let point = Point::new(x, y);
                let square = board.at(&point).unwrap();
                if square.clone().count() == 0 {
                    self.possible_moves_place(board, point, &mut moves);
                } else if square.mover() == Some(self.player) {
                    self.possible_moves_slide(board, point, &mut moves);
                }
            }
        }
        moves
    }

    fn possible_moves_place<T: Board>(&self, board: &T, point: Point, moves: &mut Vec<Turn>) {
        if !board.count().used_up(&Piece::new(Stone::Flat, self.player)) {
            moves.push(Place { point: point, stone: Stone::Flat });
            moves.push(Place { point: point, stone: Stone::Standing });
        }
        if !board.count().used_up(&Piece::new(Stone::Capstone, self.player)) {
            moves.push(Place { point: point, stone: Stone::Capstone });
        }
    }

    fn possible_moves_slide<T: Board>(&self, board: &T, point: Point, moves: &mut Vec<Turn>) {
        let pile_height = board.at(&point).unwrap().count();
        for dir in Direction::all() {
            let mut clear = 0;
            while let Some(point) = dir.adjust(&point, clear + 1, board.size()) {
                // Deal with blocking in and capstones flattening
                if let Some(piece) = board.at(&point).unwrap().last() {
                    if piece.stone() != Stone::Flat {
                        break;
                    }
                }
                clear += 1;
            }
            if clear > 0 {
                for d in self.bounded_slide(pile_height, clear) {
                    // Discard the non-move (drops.contents[0] == height)
                    if d.contents[0] < pile_height {
                        let num = pile_height - d.contents[0];
                        let drops = d.contents.into_iter().skip(1).collect::<Vec<usize>>();
                        moves.push(Slide {
                            num_pieces: num,
                            point: point,
                            direction: dir,
                            drops: drops
                        });
                    }
                }
            }
        }
    }

    fn bounded_slide(&self, height: usize, clear: usize) -> BTreeSet<Drops> {
        let mut evolving: BTreeSet<Drops> = BTreeSet::new();
        let start = Drops { last_index: 0, contents: vec![0; clear + 1] };
        evolving.insert(start);

        println!("{} {}", height, clear);
        for _ in 0..height {
            let mut evolving_next = BTreeSet::new();
            for vec in evolving {
                evolving_next.insert(add_to_last(&vec));
                if let Some(v) = add_to_length(&vec) {
                    evolving_next.insert(v);
                }
            }
            evolving = evolving_next;
        }
        println!("{:?}", evolving);
        evolving
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Drops {
    last_index: usize,
    contents: Vec<usize>,
}

fn add_to_last(v: &Drops) -> Drops {
    let mut vec = (*v).clone();
    vec.contents[vec.last_index] += 1;
    vec
}

fn add_to_length(v: &Drops) -> Option<Drops>{
    let mut vec = v.clone();
    if vec.last_index < vec.contents.len() - 1 {
        vec.contents[vec.last_index + 1] = 1;
        vec.last_index += 1;
        Some(vec)
    } else {
        None
    }
}

pub fn advantage(board: NaiveBoard) -> i64 {
//Relevant stats
//flat score
//road options by length (how to compute?)
//towers controlled (by num allied pieces in carry limit)
    0
}




