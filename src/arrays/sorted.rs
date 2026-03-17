use crate::arrays::{Array, ArrayError, INVARIANT_VIOLATED_ELEMENT_MUST_EXIST};

pub struct SortedArray<T: Ord> {
    data: Box<[Option<T>]>,
    size: usize,
}

impl<T: Ord> SortedArray<T> {
    pub fn new(capacity: usize) -> Self {
        let data = std::iter::repeat_with(|| None)
            .take(capacity)
            .collect::<Vec<_>>()
            .into_boxed_slice();

        Self { data, size: 0 }
    }

    fn lower_bound(&self, item: &T) -> usize {
        let mut left = 0usize;
        let mut right = self.size; // exclusive

        while left < right {
            let mid = left + (right - left) / 2;
            let mid_item = self.data[mid]
                .as_ref()
                .expect(INVARIANT_VIOLATED_ELEMENT_MUST_EXIST);

            if mid_item < item {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        left
    }
}

impl<T: Ord> Array<T> for SortedArray<T> {
    fn insert(&mut self, item: T) -> Result<(), ArrayError> {
        if self.size == self.data.len() {
            return Err(ArrayError::Full);
        }

        let index = self.lower_bound(&item);

        // shift right: [index..size] -> [index+1..size+1]
        for i in (index..self.size).rev() {
            self.data[i + 1] = self.data[i].take();
        }

        self.data[index] = Some(item);
        self.size += 1;
        Ok(())
    }

    fn delete(&mut self, index: usize) -> Result<T, ArrayError> {
        if self.size == 0 {
            return Err(ArrayError::Empty);
        }
        if index >= self.size {
            return Err(ArrayError::IndexOutOfBounds);
        }

        let item = self.data[index]
            .take()
            .expect(INVARIANT_VIOLATED_ELEMENT_MUST_EXIST);

        // shift left: [index+1..size] -> [index..size-1]
        for i in index..(self.size - 1) {
            self.data[i] = self.data[i + 1].take();
        }
        self.data[self.size - 1] = None;
        self.size -= 1;

        Ok(item)
    }

    fn find(&self, item: &T) -> Option<usize>
    where
        T: PartialEq,
    {
        if self.size == 0 {
            return None;
        }

        let index = self.lower_bound(item);
        if index < self.size && self.data[index].as_ref() == Some(item) {
            Some(index)
        } else {
            None
        }
    }

    fn get(&self, index: usize) -> Option<&T> {
        if index >= self.size {
            return None;
        }
        self.data[index].as_ref()
    }

    fn size(&self) -> usize {
        self.size
    }

    fn capacity(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use super::SortedArray;
    use crate::arrays::{Array, ArrayError};

    #[test]
    fn new_has_zero_size_and_capacity() {
        let arr: SortedArray<i32> = SortedArray::new(3);
        assert_eq!(arr.size(), 0);
        assert_eq!(arr.capacity(), 3);
        assert!(arr.is_empty());
    }

    #[test]
    fn insert_keeps_array_sorted() {
        let mut arr = SortedArray::new(10);
        arr.insert(5).unwrap();
        arr.insert(1).unwrap();
        arr.insert(3).unwrap();
        arr.insert(2).unwrap();
        arr.insert(4).unwrap();

        let got: Vec<i32> = (0..arr.size()).map(|i| *arr.get(i).unwrap()).collect();
        assert_eq!(got, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn insert_returns_full_error() {
        let mut arr = SortedArray::new(2);
        arr.insert(2).unwrap();
        arr.insert(1).unwrap();
        assert_eq!(arr.insert(3), Err(ArrayError::Full));
    }

    #[test]
    fn find_uses_sorted_property() {
        let mut arr = SortedArray::new(5);
        arr.insert(10).unwrap();
        arr.insert(30).unwrap();
        arr.insert(20).unwrap();

        assert_eq!(arr.find(&10), Some(0));
        assert_eq!(arr.find(&20), Some(1));
        assert_eq!(arr.find(&30), Some(2));
        assert_eq!(arr.find(&99), None);
    }

    #[test]
    fn delete_shifts_left_and_keeps_sorted() {
        let mut arr = SortedArray::new(5);
        arr.insert(1).unwrap();
        arr.insert(2).unwrap();
        arr.insert(3).unwrap();
        arr.insert(4).unwrap();

        let deleted = arr.delete(1).unwrap(); // delete 2
        assert_eq!(deleted, 2);

        let got: Vec<i32> = (0..arr.size()).map(|i| *arr.get(i).unwrap()).collect();
        assert_eq!(got, vec![1, 3, 4]);
    }

    #[test]
    fn delete_errors_on_empty_and_oob() {
        let mut arr = SortedArray::<i32>::new(3);
        assert_eq!(arr.delete(0), Err(ArrayError::Empty));

        arr.insert(1).unwrap();
        assert_eq!(arr.delete(1), Err(ArrayError::IndexOutOfBounds));
    }
}

