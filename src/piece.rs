use std::fmt;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Stone {
    Flat,
    Standing,
    Capstone,
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


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Player {
    One,
    Two,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Player::One => write!(f, "1"),
            &Player::Two => write!(f, "2"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Piece {
    pub stone: Stone,
    pub owner: Player,
}

impl Piece {
    // Flatten a standing stone if a capstone moves onto it
    pub fn move_onto(&self, base: &mut Piece) -> () {
        assert!(base.stone != Stone::Standing || self.stone == Stone::Capstone,
                "Cannot move normal stone onto standing stone");
        if base.stone == Stone::Standing && self.stone == Stone::Capstone {
            base.stone = Stone::Flat;
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.stone, self.owner)
    }
}

impl FromStr for Piece {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 4 {
            return Err(());
        }
        let mut chars = s.chars();
        let turn = chars.nth(2).unwrap();
        if "FSC".contains(turn) {
            let stone = match turn {
                'F' => Stone::Flat,
                'S' => Stone::Standing,
                'C' => Stone::Capstone,
                _ => return Err(()),
            };
            let player = match chars.next() {
                Some('1') => Player::One,
                Some('2') => Player::Two,
                _ => return Err(()),
            };

            Ok(Piece {
                stone: stone,
                owner: player,
            })
        } else {
            Err(())
        }
    }
}
