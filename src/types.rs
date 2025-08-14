#[derive(Debug)]
pub enum Part {
    One,
    Two,
    Three,
}

#[derive(Debug)]
pub struct PuzzleKey {
    pub event: i16,
    pub quest: i8,
    pub part: Part,
}
