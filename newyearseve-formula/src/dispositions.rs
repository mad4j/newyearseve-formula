use intpackit::unpack;

#[derive(Debug, Clone)]
pub struct Dispositions<I: Iterator> {
    elems: Vec<I::Item>,
    length: usize,
    first_index: usize,
    last_index: usize,
    index: usize,
}

impl<I> Iterator for Dispositions<I>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let &mut Dispositions {
            ref elems,
            ref length,
            first_index: _,
            ref mut last_index,
            ref mut index,
        } = self;

        if index < last_index {
            // compute next value
            let result = Some(
                unpack(*index, elems.len(), *length)
                    .map(|x| elems[x].clone())
                    .collect(),
            );

            // update next value index
            *index += 1;

            // return result value
            result
        } else {
            // iterator end reached
            None
        }
    }
}

impl<I> ExactSizeIterator for Dispositions<I>
where
    I: Iterator,
    I::Item: Clone,
{
    fn len(&self) -> usize {
        self.last_index - self.first_index
    }
}

pub fn dispositions<I: Iterator>(iter: I, k: usize) -> Dispositions<I> {
    dispositions_part(iter, k, 0, 1)
}

pub fn dispositions_part<I: Iterator>(iter: I, k: usize, part: u8, parts: u8) -> Dispositions<I> {
    let elems: Vec<I::Item> = iter.collect();
    let max_index: usize = if k == 0 { 0 } else { elems.len().pow(k as u32) };

    let part_size: f32 = max_index as f32 / parts as f32;

    let first_index = (part as f32 * part_size) as usize;
    let last_index = ((part + 1) as f32 * part_size) as usize;

    Dispositions {
        elems,
        length: k,
        first_index,
        last_index,
        index: first_index,
    }
}

pub trait DispositionsTrait: Sized + Iterator {
    fn dispositions(self, k: usize) -> Dispositions<Self>;
    fn dispositions_part(self, k: usize, part: u8, parts: u8) -> Dispositions<Self>;
}

impl<I: Iterator> DispositionsTrait for I {
    fn dispositions(self, k: usize) -> Dispositions<Self> {
        dispositions(self, k)
    }
    fn dispositions_part(self, k: usize, part: u8, parts: u8) -> Dispositions<Self> {
        dispositions_part(self, k, part, parts)
    }
}
