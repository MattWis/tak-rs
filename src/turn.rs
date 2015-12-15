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

impl FromStr for Turn {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let places = "abcdefgh";
        let slides = "012345678";

        let first = s.chars().nth(0).unwrap();
        if places.chars().position(|x| x == first).is_some() {
            // My placement method - See README
            let point = try!(s[0..2].parse::<Point>());
            let piece = try!(s[2..4].parse::<Piece>());
            Ok(Turn::Place {
                point: point,
                piece: piece,
            })
        } else if let Some(pieces) = slides.chars().position(|x| x == first) {
            // Slide - PTN Notation
            let point = try!(s[1..3].parse::<Point>());
            let direction = match s.chars().nth(3) {
                Some('>') => Direction::Right,
                Some('<') => Direction::Left,
                Some('+') => Direction::Up,
                Some('-') => Direction::Down,
                _ => return Err(()),
            };
            let drops = s.chars().skip(4).map(|c| {
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
