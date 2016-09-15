use std::str::FromStr;
use std::fmt;

use point::Point;
use piece::Stone;

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

    pub fn all() -> Vec<Direction> {
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

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Direction::Right => write!(f, ">"),
            &Direction::Left => write!(f, "<"),
            &Direction::Up => write!(f, "+"),
            &Direction::Down => write!(f, "-"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, RustcDecodable, RustcEncodable)]
pub enum Turn {
    Place {
        point: Point,
        stone: Stone,
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
        // Pay a runtime cost for String slices to guarantee no panics
        fn slice(s: &str, start: usize, end: usize) -> String {
            s.chars().skip(start).take(end - start).collect::<String>()
        }

        // Used for Turn::Slide
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

        if let Ok(stone) = slice(s, 0, 1).parse::<Stone>() {
            // Placement - PTN notation Full
            let point = try!(slice(s, 1, 3).parse::<Point>());
            Ok(Turn::Place {
                point: point,
                stone: stone,
            })
        } else if let Ok(pieces) = s[0..1].parse::<usize>() {
            // Slide - PTN Notation Full
            let point = try!(slice(s, 1, 3).parse::<Point>());
            let direction = try!(slice(s, 3, 4).parse::<Direction>());
            let drops = try!(get_drops(&slice(s, 4, s.len())));
            Ok(Turn::Slide {
                num_pieces: pieces,
                point: point,
                direction: direction,
                drops: drops,
            })
        } else if let Ok(point) = slice(s, 0, 2).parse::<Point>() {
            if let Ok(direction) = slice(s, 2, 3).parse::<Direction>() {
                // Slide - abbreviated
                let drops = try!(get_drops(&slice(s, 3, s.len())));
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
            } else {
                // Place - abbreviated
                Ok(Turn::Place {
                    point: point,
                    stone: Stone::Flat,
                })
            }
        } else {
            Err(())
        }
    }
}

impl fmt::Display for Turn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Turn::Place { ref point, ref stone } => {
                write!(f, "{}{}", stone, point)
            },
            &Turn::Slide { ref num_pieces, ref point, ref direction, ref drops } => {
                try!(write!(f, "{}{}{}", num_pieces, point, direction));
                for x in drops {
                    try!(write!(f, "{}", x));
                }
                write!(f, "")
            }
        }
    }
}
