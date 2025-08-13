pub enum Part {
    One,
    Two,
    Three,
}

pub struct PuzzleKey {
    pub event: i16,
    pub quest: i8,
    pub part: Part,
}