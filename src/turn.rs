use std::str::FromStr;

use point::Point;
use piece::Piece;

#[derive(Clone, Copy, Debug, PartialEq, Eq, RustcDecodable, RustcEncodable)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl Direction {
    pub fn adjust(&self, point: &Point, offset: usize, size: usize) -> Option<Point> {
        match self {
            &Direction::Right => if point.x + offset < size {
                Some(Point {
                    x: point.x + offset,
                    y: point.y,
                })
            } else {
                None
            },
            &Direction::Left => if point.x >= offset {
                Some(Point {
                    x: point.x - offset,
                    y: point.y,
                })
            } else {
                None
            },
            &Direction::Up => if point.y + offset < size {
                Some(Point {
                    x: point.x,
                    y: point.y + offset,
                })
            } else {
                None
            },
            &Direction::Down => if point.y >= offset {
                Some(Point {
                    x: point.x,
                    y: point.y - offset,
                })
            } else {
                None
            },
        }
    }

    fn all() -> Vec<Direction> {
        vec![Direction::Right, Direction::Left, Direction::Down, Direction::Up]
    }

    /// Gives all of the neighbors of point, assuming a board size
    pub fn neighbors(point: &Point, size: usize) -> Vec<Point> {
        Direction::all().iter()
                        .filter_map(|d| d.adjust(point, 1, size))
                        .collect::<Vec<_>>()
    }
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            Err(())
        } else {
            match s.chars().nth(0) {
                Some('>') => Ok(Direction::Right),
                Some('<') => Ok(Direction::Left),
                Some('+') => Ok(Direction::Up),
                Some('-') => Ok(Direction::Down),
                _ => return Err(()),
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, RustcDecodable, RustcEncodable)]
pub enum Turn {
    Place {
        point: Point,
        piece: Piece,
    },
    Slide {
        num_pieces: usize,
        point: Point,
        direction: Direction,
        drops: Vec<usize>,
    },
}

pub type TurnErr = ();

impl FromStr for Turn {
    type Err = TurnErr;
    fn from_str(s: &str) -> Result<Self, TurnErr> {
        fn get_drops(s: &str) -> Result<Vec<usize>, TurnErr> {
            let drops = s.chars().map(|c| {
                                   match c.to_digit(10) {
                                       Some(x) => x as usize,
                                       None => 100, // Bad error handling...
                                   }
                               })
                               .collect::<Vec<_>>();
            if drops.iter().any(|x| *x > 99) {
                return Err(());
            }
            if drops.iter().any(|x| *x < 1) {
                return Err(());
            }
            Ok(drops)
        }

        let places = "abcdefgh";
        let slides = "012345678";

        let first = s.chars().nth(0).unwrap();
        if places.chars().position(|x| x == first).is_some() {
            // My placement method - See README
            let point = try!(s[0..2].parse::<Point>());
            if let Ok(piece) = s[2..].parse::<Piece>() {
                Ok(Turn::Place {
                    point: point,
                    piece: piece,
                })
            } else {
                // Slide - abbreviated
                let direction = try!(s[2..3].parse::<Direction>());
                let drops = try!(get_drops(&s[3..]));
                if drops.len() > 0 {
                   Ok(Turn::Slide {
                       num_pieces: 1,
                       point: point,
                       direction: direction,
                       drops: drops,
                   })
                } else {
                   Ok(Turn::Slide {
                       num_pieces: 1,
                       point: point,
                       direction: direction,
                       drops: vec![1],
                   })
                }
            }
        } else if let Some(pieces) = slides.chars().position(|x| x == first) {
            // Slide - PTN Notation Full
            let point = try!(s[1..3].parse::<Point>());
            let direction = try!(s[3..4].parse::<Direction>());
            let drops = try!(get_drops(&s[4..]));
            Ok(Turn::Slide {
                num_pieces: pieces,
                point: point,
                direction: direction,
                drops: drops,
            })
        } else {
            Err(())
        }
    }
}
