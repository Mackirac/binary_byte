use std::fmt::{ self, Formatter, Debug };
use std::ops::{ Index, IndexMut };

/// A binary representation of a byte.
#[derive(PartialEq)]
pub struct ByteBase2 {
    intern: [bool;8]
}

/// Error occurred when trying to construct a ByteBase2 with an invalid string pattern.
/// 
/// # Example
/// 
/// ```rust
/// use binary_byte::{ ByteBase2, InvalidPattern };
/// 
/// assert_eq!(ByteBase2::from_string("foo"), Err(InvalidPattern));
/// ```
/// 
/// See also [ByteBase2::from_string](struct.ByteBase2.html#method.from_string).
#[derive(PartialEq)]
pub struct InvalidPattern;

impl Debug for InvalidPattern {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "The string pattern should have exactly 8, '0' or '1', characters.")
    }
}

impl  ByteBase2 {
    /// Returns how many ones there is in this byte.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use binary_byte::ByteBase2;
    /// 
    /// let byte = ByteBase2::from_dec(15);
    /// assert_eq!(byte.ones(), 4);
    /// ```
    pub fn ones(&self) -> usize {
        self.intern.iter().filter(|bit| **bit).count()
    }

    /// Returns an iterator over this byte's bits.
    /// 
    /// Yields first the least significative bit and last the most significative one.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use binary_byte::ByteBase2;
    /// 
    /// let byte = ByteBase2::from_string("00000011").unwrap();
    /// let mut byte_iter = byte.iter();
    /// assert_eq!(byte_iter.next(), Some(true));
    /// assert_eq!(byte_iter.next(), Some(true));
    /// assert_eq!(byte_iter.next(), Some(false));
    /// ```
    pub fn iter(&self) -> impl Iterator<Item=bool> {
        Vec::from(self.intern.as_ref()).into_iter()
    }

    /// Tries to create a ByteBase2 from a string representing an 8 bit binary number.
    /// 
    /// # Errors
    /// Returns an Err([InvalidPattern](struct.InvalidPattern.html)) if the pattern length is not exactly 8 or
    /// if any of its characters is different of '1' or '0'.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use binary_byte::{ ByteBase2, InvalidPattern };
    /// 
    /// assert!(ByteBase2::from_string("01111001").is_ok());
    /// assert_eq!(ByteBase2::from_string("12001101"), Err(InvalidPattern));
    /// assert_eq!(ByteBase2::from_string("101010100"), Err(InvalidPattern));
    /// assert_eq!(ByteBase2::from_string("1010"), Err(InvalidPattern));
    /// ```
    pub fn from_string(pattern: impl Into<String>) -> Result<Self, InvalidPattern> {
        let pattern = pattern.into();
        if pattern.len() == 8 {
            let mut intern = [false;8];
            for (index, bit) in pattern.chars().rev().enumerate() {
                if bit == '1' { intern[index] = true; }
                else if bit != '0' { return Err(InvalidPattern); }
            }
            return Ok(ByteBase2 { intern });
        }
        Err(InvalidPattern)
    }

    /// Creates a ByteBase2 object from an u8 value.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use binary_byte::ByteBase2;
    /// 
    /// let byte = ByteBase2::from_dec(15);
    /// assert_eq!(format!("{:?}", byte), "00001111".to_string());
    /// ```
    pub fn from_dec(mut input: u8) -> Self {
        let mut intern = [false;8];
        for index in 0..8 {
            intern[index] = input % 2 == 1;
            input /= 2;
        }
        ByteBase2 { intern }
    }

    /// Converts a ByteBase2 number to its decimal representation.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use binary_byte::ByteBase2;
    /// 
    /// let byte = ByteBase2::from_string("00001000").unwrap();
    /// assert_eq!(byte.as_dec(), 8);
    /// ```
    pub fn as_dec(&self) -> u8 {
        let mut output = 0;
        for index in 0..8 {
            if self.intern[index] {
                output += 2_u8.pow(index as u32);
            }
        }
        output
    }
}

/// Access the bits in this byte.
/// 
/// Index 0 access the least significative bit.
impl Index<usize> for ByteBase2 {
    type Output = bool;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.intern[idx]
    }
}

/// Mutably access the bits in this byte.
/// 
/// Index 0 access the least significative bit.
impl IndexMut<usize> for ByteBase2 {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.intern[idx]
    }
}

impl Debug for ByteBase2 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut output = String::with_capacity(8);
        for bit in self.intern.iter().rev() {
            if *bit { output.push('1'); }
            else { output.push('0'); }
        }
        write!(f, "{}", output)
    }
}

#[cfg(test)]
mod test_mod {
    use crate::ByteBase2;

    #[test]
    #[should_panic]
    fn index_test() {
        let byte = ByteBase2::from_dec(15);
        byte[8];
    }

    #[test]
    fn debug_test() {
        assert_eq!(format!("{:?}", ByteBase2::from_dec(15)), "00001111".to_string());
    }
}
