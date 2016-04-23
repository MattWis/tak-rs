use std::fmt;
use std::str::FromStr;

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq, Eq, RustcDecodable, RustcEncodable)]
pub enum Stone {
    Flat = 1,
    Standing = 2,
    Capstone = 3,
}
}

impl fmt::Display for Stone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Stone::Flat => write!(f, "F"),
            &Stone::Standing => write!(f, "S"),
            &Stone::Capstone => write!(f, "C"),
        }
    }
}

impl FromStr for Stone {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stone_type = s.chars().nth(0).unwrap_or('0');
        if "FSC".contains(stone_type) {
            match stone_type {
                'F' => Ok(Stone::Flat),
                'S' => Ok(Stone::Standing),
                'C' => Ok(Stone::Capstone),
                _ => Err(()),
            }
        } else {
            Err(())
        }
    }
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug, PartialEq, Eq, RustcDecodable, RustcEncodable)]
pub enum Player {
    One = 1,
    Two = 0,
}
}

impl Player {
    pub fn other(&self) -> Player {
        match self {
            &Player::One => Player::Two,
            &Player::Two => Player::One,
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Player::One => write!(f, "1"),
            &Player::Two => write!(f, "2"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, RustcDecodable, RustcEncodable)]
pub struct Piece {
    pub stone: Stone,
    pub owner: Player,
}

impl Piece {
    pub fn new(stone: Stone, owner: Player) -> Piece {
        Piece { stone: stone, owner: owner }
    }

    // Flatten a standing stone if a capstone moves onto it
    // Cannot move onto capstone or standing stone otherwise
    pub fn move_onto(&self, base: &mut Piece) -> Result<(), &str> {
        if base.stone == Stone::Capstone {
            return Err("Cannot move onto Capstone");
        }
        if base.stone == Stone::Standing && self.stone != Stone::Capstone {
            return Err("Cannot move normal stone onto standing stone");
        }
        if base.stone == Stone::Standing && self.stone == Stone::Capstone {
            base.stone = Stone::Flat;
        }
        Ok(())
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.stone, self.owner)
    }
}
