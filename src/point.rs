use std::str::FromStr;
use std::fmt;

// TODO: Optimize to a single u8
// How to handle board sizes? (Maybe 2 u8s)
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, RustcDecodable, RustcEncodable)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Point {
        Point { x: x, y: y }
    }
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 2 {
            return Err(());
        }
        let mut chars = s.chars();
        let letters = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        let numbers = ['1', '2', '3', '4', '5', '6', '7', '8'];
        let grid_x = chars.next().unwrap_or('0');
        let grid_y = chars.next().unwrap_or('0');
        let x = match letters.iter().position(|c| *c == grid_x) {
            Some(num) => num,
            None => return Err(()),
        };
        let y = match numbers.iter().position(|c| *c == grid_y) {
            Some(num) => num,
            None => return Err(()),
        };
        Ok(Point { x: x, y: y })
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let letters = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        let numbers = ['1', '2', '3', '4', '5', '6', '7', '8'];
        try!(write!(f, "{}", letters[self.x]));
        write!(f, "{}", numbers[self.y])
    }
}
