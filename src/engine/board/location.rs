use strum_macros::FromRepr;

#[derive(FromRepr, Debug, Copy, Clone)]
pub enum File{
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

#[derive(FromRepr, Debug, Copy, Clone)]
pub enum Rank{
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct Location{
    pub file: File,
    pub rank: Rank,
}

impl Location{
    pub(crate) const fn new(file: File, rank: Rank) -> Self {
        Self{file, rank}
    }
}