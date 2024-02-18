use bytemuck::Pod;

const POOL_EXPANSION_MODIFIER: usize = 2;

pub struct PodPool<T: Pod + Default> {
    buffer: Vec<T>,
    aviable_indexes: Vec<usize>
}

impl<T: Pod + Default> PodPool<T> {
    pub fn pull(&mut self) -> usize {
        if self.aviable_indexes.is_empty() {
            self.resize(self.buffer.capacity() * POOL_EXPANSION_MODIFIER);
        }
        self.aviable_indexes.pop().unwrap()
    }

    pub fn free(&mut self, index: usize) {
        self.aviable_indexes.push(index)
    }

    fn resize(&mut self, length: usize) {
        if length < self.buffer.capacity() {
            panic!("Reducing pool capacity is prohibited")
        }
        let last_capacity = self.buffer.capacity();
        self.buffer.resize_with(length, || T::default());
        for i in last_capacity..length-1 {
            self.aviable_indexes.push(i);
        }
    }
}
