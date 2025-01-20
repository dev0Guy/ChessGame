use std::fmt;

pub(crate) struct BitBoard(u64);


impl BitBoard{
    /// Creates a new `BitBoard` with the given bitboard value.
    ///
    /// # Parameters
    /// - `v`: A `u64` value representing the bitboard, where each bit corresponds to a square
    ///        on the chessboard. Bit 0 represents `a1`, and bit 63 represents `h8`.
    ///
    /// # Returns
    /// A `BitBoard` initialized with the provided value.
    pub fn new(v: u64) -> Self {
        Self(v)
    }

    /// Creates an empty `BitBoard` where all squares are unoccupied.
    ///
    /// # Returns
    /// A `BitBoard` with all bits set to `0`, representing an empty chessboard.
    pub fn empty() -> Self {
        Self(0)
    }
}

impl fmt::Debug for BitBoard{
    // Formats the `BitBoard` in a human-readable chessboard representation.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Chessboard visualization (LSB is a1):")?;
        for rank in (0..8).rev() {
            for file in 0..8 {
                let square_index = rank * 8 + file;
                let mask = 1u64 << square_index;
                if self.0 & mask != 0 {
                    write!(f, "X ")?;
                } else {
                    write!(f, ". ")?;
                }
            }
            write!(f, "{} ", rank + 1)?;
            writeln!(f)?;
        }
        writeln!(f, "a b c d e f g h")
    }
}
