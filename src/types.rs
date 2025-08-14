#[derive(Debug)]
pub enum Part {
    One,
    Two,
    Three,
}

impl Part {
    pub fn as_u8(&self) -> u8 {
        match *self {
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
        }
    }
}

#[derive(Debug)]
pub struct PuzzleKey {
    pub event: i16,
    pub quest: i8,
    pub part: Part,
}
