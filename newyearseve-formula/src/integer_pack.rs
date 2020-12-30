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

/// Extract integer values packed in a single integer
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
