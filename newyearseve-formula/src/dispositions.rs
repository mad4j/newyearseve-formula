
#[path = "unpack.rs"]
mod unpack;
use unpack::UnpackIterator;


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
        if self.index < self.max_index {
            let &mut Dispositions {
                ref elems,
                ref length,
                max_index: _,
                ref mut index,
            } = self;

            *index += 1;

            Some(
                UnpackIterator::init(*index, elems.len(), *length)
                    .map(|x| self.elems[x].clone())
                    .collect(),
            )
        } else {
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