#[derive(Clone, Copy, Eq, PartialEq, Debug, Default)]
pub struct BitSet {
    data: u32,
}

impl BitSet {
    pub const fn new() -> BitSet {
        BitSet { data: 0 }
    }
    pub const fn with_bits(bits: &[u8]) -> BitSet {
        let mut data = 0u32;
        let mut i = 0;
        // Using while to make the fn const
        while i < bits.len() {
            data |= 1 << bits[i];
            i += 1;
        }

        BitSet { data }
    }
    pub fn has(self, bit: u8) -> bool {
        (self.data & (1u32 << bit)) != 0
    }
}

impl IntoIterator for BitSet {
    type Item = u8;
    type IntoIter = BitSetIterator;

    fn into_iter(self) -> Self::IntoIter {
        BitSetIterator::new(self)
    }
}

#[derive(Clone, Copy)]
pub struct BitSetIterator {
    set: BitSet,
    i: i8,
}

impl BitSetIterator {
    fn new(set: BitSet) -> BitSetIterator {
        BitSetIterator { set, i: -1 }
    }
}

impl Iterator for BitSetIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        while self.i < 30 {
            self.i += 1;
            if self.set.has(self.i as u8) {
                return Some(self.i as u8);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_set() {
        let set = BitSet::with_bits(&[2, 3, 5, 6, 7, 8, 9, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]);
        assert!(!set.has(0));
        assert!(!set.has(1));
        assert!(set.has(2));
        assert!(set.has(3));
        assert!(!set.has(4));
        assert!(set.has(5));
        assert!(set.has(6));
        assert!(set.has(7));
        assert!(set.has(8));
        assert!(set.has(9));
        assert!(!set.has(10));
        assert!(set.has(11));
        assert!(set.has(12));
        assert!(set.has(13));
        assert!(set.has(14));
        assert!(set.has(15));
        assert!(set.has(16));
        assert!(set.has(17));
        assert!(set.has(18));
        assert!(set.has(19));
        assert!(set.has(20));
        assert!(!set.has(21));
    }

    #[test]
    fn test_bit_set_iterator() {
        let set = BitSet::with_bits(&[2, 3, 5, 6, 7, 8, 9, 11]);
        let mut iter = set.into_iter();
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.next(), Some(6));
        assert_eq!(iter.next(), Some(7));
        assert_eq!(iter.next(), Some(8));
        assert_eq!(iter.next(), Some(9));
        assert_eq!(iter.next(), Some(11));
        assert_eq!(iter.next(), None);
    }
}
