use std::str::FromStr;

use piece;
use point;

#[derive(Debug)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl Direction {
    pub fn adjust(&self, point: &point::Point, offset: usize) -> point::Point {
        match self {
            &Direction::Right => point::Point {
                x: point.x + offset,
                y: point.y,
            },
            &Direction::Left => point::Point {
                x: point.x - offset,
                y: point.y,
            },
            &Direction::Up => point::Point {
                x: point.x,
                y: point.y + offset,
            },
            &Direction::Down => point::Point {
                x: point.x,
                y: point.y - offset,
            },
        }
    }
}

#[derive(Debug)]
pub enum Turn {
    Placement {
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
        let point = match s.parse::<point::Point>() {
            Ok(p) => p,
            Err(_) => return Err(()),
        };
        let mut chars = s.chars();
        let turn = chars.nth(2).unwrap();
        if let Ok(piece) = s.parse::<piece::Piece>() {
            Ok(Turn::Placement {
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
            let offsets = chars.map(|c| c.to_digit(10).unwrap() as usize)
                               .collect();

            Ok(Turn::Slide {
                point: point,
                direction: direction,
                offsets: offsets,
            })
        }
    }
}
