use strum_macros::FromRepr;

#[derive(FromRepr, Debug, Copy, Clone)]
pub enum File {
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
pub enum Rank {
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
pub(crate) struct Location {
    pub file: File,
    pub rank: Rank,
}

impl File {
    pub fn from_char(c: char) -> Option<Self> {
        let index = (c.to_ascii_lowercase() as u8).checked_sub(b'a')? as usize;
        Self::from_repr(index)
    }
}

impl Rank {

    pub fn from_char(c: char) -> Option<Self> {
       Self::from_repr(8 - c.to_digit(10).unwrap() as usize)
    }
}

impl Location {
    pub(crate) const fn new(file: File, rank: Rank) -> Self {
        Self { file, rank }
    }

    pub(crate) fn from(value: &str) -> Result<Self, ()> {
        match value.chars().collect::<Vec<char>>().as_slice() {
            [first, second, ..] => {
                let file = File::from_char(*first).unwrap();
                let rank = Rank::from_char(*second).unwrap();
                Ok(Location::new(file, rank))
            },
            _ => Err(())
        }
    }
}
