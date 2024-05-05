pub struct Cartesian<T> {
    slices: Box<[CartesianSlice<T>]>,
    done: bool,
}

struct CartesianSlice<T> {
    slice: Box<[T]>,
    index: usize,
}

impl<T: Clone> Cartesian<T> {
    fn new(slices: &[Box<[T]>]) -> Self {
        let slices: Vec<_> = slices
            .iter()
            .map(|slice| CartesianSlice {
                slice: slice.clone(),
                index: 0,
            })
            .collect();
        let slices = slices.into_boxed_slice();
        Cartesian { slices, done: false }
    }

    fn advance_once(&mut self) {
        self.done = {
            let mut i = 0;
            loop {
                if i >= self.slices.len() {
                    break true;
                }
                let slice = &mut self.slices[i];
                slice.index += 1;
                if slice.index >= slice.slice.len() {
                    slice.index = 0;
                    i += 1;
                } else {
                    break false;
                }
            }
        };
    }
}

impl<T: Clone> Iterator for Cartesian<T> {
    type Item = Box<[T]>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else {
            let items: Vec<_> = self.slices.iter().map(|s| s.slice[s.index].clone()).collect();
            let items = items.into_boxed_slice();
            self.advance_once();
            Some(items)
        }
    }
}

pub fn cartesian<T: Clone>(slices: &[Box<[T]>]) -> Cartesian<T> {
    Cartesian::new(slices)
}
