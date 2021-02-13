#[derive(Debug)]
pub struct IntegerPack {
    // packed values
    value: usize,
    // max value
    modulo: usize,
    // number of packed values
    length: usize,
    // next value to be extraced
    index: usize,
}

/// Extract integer values packed in a single number
pub fn unpack(value: usize, modulo: usize, length: usize) -> IntegerPack {
    // intialize a new structure
    IntegerPack {
        value,
        modulo,
        length,
        index: 0,
    }
}

impl Iterator for IntegerPack {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        // shortcuts to internal status
        let &mut IntegerPack {
            ref mut value,
            ref modulo,
            ref mut length,
            ref mut index,
        } = self;

        if index < length {
            // compute next value
            let v = *value % modulo;

            // update internal state
            *index += 1;
            *value /= modulo;

            Some(v)
        } else {
            // end reached
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer_pack_test_01() {
        let mut ip = unpack(0b11_00_01_10_11_00, 4, 6);
        assert_eq!(ip.next(), Some(0));
        assert_eq!(ip.next(), Some(3));
        assert_eq!(ip.next(), Some(2));
        assert_eq!(ip.next(), Some(1));
        assert_eq!(ip.next(), Some(0));
        assert_eq!(ip.next(), Some(3));
        assert_eq!(ip.next(), None);
    }

    #[test]
    fn integer_pack_test_02() {
        let mut ip = unpack(1 + 2 * 3 + 0 * 9 + 2 * 27 + 1 * 81, 3, 6);
        assert_eq!(ip.next(), Some(1));
        assert_eq!(ip.next(), Some(2));
        assert_eq!(ip.next(), Some(0));
        assert_eq!(ip.next(), Some(2));
        assert_eq!(ip.next(), Some(1));
        assert_eq!(ip.next(), Some(0));
        assert_eq!(ip.next(), None);
    }
}
