use std::{fmt, ops};

#[derive(Copy, Clone, PartialEq)]
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

    /// Returns a `BitBoard` with its bits reversed.
    pub fn reverse(&self) -> Self {
        BitBoard(self.0.reverse_bits())
    }

    pub fn is_empty(&self) -> bool{
        self.0 == 0
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

impl ops::BitOr for BitBoard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 | rhs.0)
    }
}

impl<'a, 'b> ops::BitOr<&'b BitBoard> for &'a BitBoard {
    type Output = BitBoard;

    fn bitor(self, rhs: &BitBoard) -> Self::Output {
        BitBoard(self.0 | rhs.0)
    }
}

impl ops::BitAnd for BitBoard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 & rhs.0)
    }
}

impl<'a, 'b> ops::BitAnd<&'b BitBoard> for &'a BitBoard {
    type Output = BitBoard;

    fn bitand(self, rhs: &BitBoard) -> Self::Output {
        BitBoard(self.0 & rhs.0)
    }
}

impl ops::BitXor for BitBoard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 ^ rhs.0)
    }
}

impl<'a, 'b> ops::BitXor<&'b BitBoard> for &'a BitBoard {
    type Output = BitBoard;

    fn bitxor(self, rhs: &BitBoard) -> Self::Output {
        BitBoard(self.0 ^ rhs.0)
    }
}

impl ops::Not for BitBoard {
    type Output = Self;

    fn not(self) -> Self::Output {
        BitBoard(!self.0)
    }
}

impl<'a> ops::Not for &'a BitBoard {
    type Output = BitBoard;

    fn not(self) -> Self::Output {
        BitBoard(!self.0)
    }
}

impl ops::Shl<u32> for BitBoard {
    type Output = Self;

    fn shl(self, rhs: u32) -> Self::Output {
        BitBoard(self.0 << rhs)
    }
}

impl<'a> ops::Shl<u32> for &'a BitBoard {
    type Output = BitBoard;

    fn shl(self, rhs: u32) -> Self::Output {
        BitBoard(self.0 << rhs)
    }
}

impl ops::Shr<u32> for BitBoard {
    type Output = Self;

    fn shr(self, rhs: u32) -> Self::Output {
        BitBoard(self.0 >> rhs)
    }
}

impl<'a> ops::Shr<u32> for &'a BitBoard {
    type Output = BitBoard;

    fn shr(self, rhs: u32) -> Self::Output {
        BitBoard(self.0 >> rhs)
    }
}

impl ops::Sub for BitBoard {
    type Output = Self;

    /// Subtracts one `BitBoard` from another.
    fn sub(self, rhs: Self) -> Self::Output {
        BitBoard(self.0.wrapping_sub(rhs.0))
    }
}

impl ops::Mul<u64> for BitBoard {
    type Output = Self;

    /// Performs multiplication between a `BitBoard` and a `u64` value.
    ///
    /// Multiplies the internal `u64` value of the `BitBoard` by the provided `u64` value.
    fn mul(self, rhs: u64) -> Self::Output {
        BitBoard(self.0.wrapping_mul(rhs))
    }
}

impl ops::BitOrAssign for BitBoard{
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl<'a> ops::BitOrAssign<&'a BitBoard> for BitBoard {
    fn bitor_assign(&mut self, rhs: &'a BitBoard) {
        self.0 |= rhs.0;
    }
}

impl ops::BitXorAssign for BitBoard{
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitand() {
        let a = BitBoard(0x00000000000000FF);
        let b = BitBoard(0x000000000000FF00);
        let expected = BitBoard(0x0000000000000000);

        assert_eq!(a & b, expected);
        assert_eq!(&a & &b, expected);
    }

    #[test]
    fn test_bitor() {
        let a = BitBoard(0x00000000000000FF);
        let b = BitBoard(0x000000000000FF00);
        let expected = BitBoard(0x000000000000FFFF);

        assert_eq!(a | b, expected);
        assert_eq!(&a | &b, expected);
    }

    #[test]
    fn test_bitxor() {
        let a = BitBoard(0x00000000000000FF);
        let b = BitBoard(0x000000000000FF00);
        let expected = BitBoard(0x000000000000FFFF);

        assert_eq!(a ^ b, expected);
        assert_eq!(&a ^ &b, expected);
    }

    #[test]
    fn test_not() {
        let a = BitBoard(0x00000000000000FF);
        let expected = BitBoard(0xFFFFFFFFFFFFFF00);

        assert_eq!(!a, expected);
        assert_eq!(!&a, expected);
    }

    #[test]
    fn test_shl() {
        let a = BitBoard(0x00000000000000FF);
        let expected = BitBoard(0x000000000000FF00);

        assert_eq!(a << 8, expected);
        assert_eq!(&a << 8, expected);
    }

    #[test]
    fn test_shr() {
        let a = BitBoard(0x000000000000FF00);
        let expected = BitBoard(0x00000000000000FF);

        assert_eq!(a >> 8, expected);
        assert_eq!(&a >> 8, expected);
    }
}