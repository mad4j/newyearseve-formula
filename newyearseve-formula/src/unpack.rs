
#[derive(Debug)]
pub struct UnpackIterator {
    value: usize,
    modulo: usize,
    length: usize,
    index: usize,
}

impl UnpackIterator {
    pub fn init(value: usize, modulo: usize, length: usize) -> UnpackIterator {
        UnpackIterator {
            value,
            modulo,
            length,
            index: 0,
        }
    }
}

impl Iterator for UnpackIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let &mut UnpackIterator {
            ref mut value,
            ref modulo,
            ref mut length,
            ref mut index,
        } = self;

        if index < length {
            let v = *value % modulo;

            *index += 1;
            *value /= modulo;

            Some(v)
        } else {
            None
        }
    }
}

