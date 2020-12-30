#[path = "integer_pack.rs"]
mod integer_pack;
use integer_pack::unpack;

#[derive(Debug, Clone)]
pub struct Dispositions<I: Iterator> {
    elems: Vec<I::Item>,
    length: usize,
    max_index: usize,
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
            ref mut max_index,
            ref mut index,
        } = self;

        if index < max_index {
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

pub fn dispositions<I: Iterator>(iter: I, k: usize) -> Dispositions<I> {
    let elems: Vec<I::Item> = iter.collect();
    let max_index: usize = if k == 0 { 0 } else { elems.len().pow(k as u32) };

    Dispositions {
        elems,
        length: k,
        max_index,
        index: 0,
    }
}

pub trait DispositionsTrait: Sized + Iterator {
    fn dispositions(self, k: usize) -> Dispositions<Self>;
}

impl<I: Iterator> DispositionsTrait for I {
    fn dispositions(self, k: usize) -> Dispositions<Self> {
        dispositions(self, k)
    }
}
