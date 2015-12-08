use std::str::FromStr;

use piece;
use point;
use point::Point;

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
        point: point::Point,
        piece: piece::Piece,
    },
    Slide {
        point: point::Point,
        direction: Direction,
        offsets: Vec<usize>,
    },
}

impl FromStr for Turn {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 4 {
            return Err(());
        }
        let point = try!(s.parse::<point::Point>());
        let mut chars = s.chars();
        let turn = match chars.nth(2) {
            Some(c) => c,
            None => return Err(()),
        };
        if let Ok(piece) = s.parse::<piece::Piece>() {
            Ok(Turn::Place {
                point: point,
                piece: piece,
            })
        } else {
            let direction = match turn {
                'R' => Direction::Right,
                'L' => Direction::Left,
                'U' => Direction::Up,
                'D' => Direction::Down,
                _ => return Err(()),
            };
            let offsets = chars.map(|c| {
                                   match c.to_digit(10) {
                                       Some(x) => x as usize,
                                       None => 100,
                                   }
                               })
                               .collect::<Vec<_>>();

            if offsets.iter().any(|x| *x > 99) {
                return Err(());
            }

            Ok(Turn::Slide {
                point: point,
                direction: direction,
                offsets: offsets,
            })
        }
    }
}
