use std::usize;

use bytemuck::Pod;

const POOL_EXPANSION_MODIFIER: usize = 2;

#[derive(Default, Debug)]
pub struct PodPool<T: Pod + Default> {
    buffer: Vec<T>,
    aviable_indexes: Vec<usize>
}

impl<T: Pod + Default> PodPool<T> {

    pub fn pull(&mut self) -> usize {
        if self.aviable_indexes.is_empty() {
            self.resize(self.buffer.capacity() * POOL_EXPANSION_MODIFIER + 1);
        }
        self.aviable_indexes.pop().unwrap()
    }

    pub fn free(&mut self, index: usize) {
        self.aviable_indexes.push(index);
        *self.get(index) = T::default();
    }

    pub fn get(&mut self, index: usize) -> &mut T {
        &mut self.buffer[index]
    }

    fn resize(&mut self, length: usize) {
        if length < self.buffer.capacity() {
            panic!("Reducing pool capacity is prohibited")
        }
        let last_capacity = self.buffer.capacity();
        self.buffer.resize_with(length, || T::default());
        for i in last_capacity..length {
            self.aviable_indexes.push(i);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PodPool;

    #[test]
    fn test_add() {
        let mut pool = PodPool::default();
        let first = pool.pull();
        *pool.get(first) = 1;
        let second = pool.pull();
        *pool.get(second) = 2;
        
        assert_eq!(*pool.get(first), 1);
        pool.free(second);
    }
}
