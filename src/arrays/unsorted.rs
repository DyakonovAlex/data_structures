use crate::arrays::{Array, ArrayError};

pub struct UnsortedArray<T> {
    data: Box<[Option<T>]>, 
    size: usize,
}

impl<T> UnsortedArray<T> {
    pub fn new(capacity: usize) -> Self {
        let data = std::iter::repeat_with(|| None)
            .take(capacity)
            .collect::<Vec<_>>()
            .into_boxed_slice();

        Self {
            data,
            size: 0,
        }
    }
}

impl<T> Array<T> for UnsortedArray<T> {
    fn insert(&mut self, item: T) -> Result<(), ArrayError> {
        if self.size == self.data.len() {
            return Err(ArrayError::Full);
        }

        self.data[self.size] = Some(item);
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

        let last_index = self.size - 1;
        let item = self
            .data[index]
            .take()
            .expect("internal invariant violated: element must exist");

        self.data[index] = self.data[last_index].take();
        self.size -= 1;

        Ok(item)
    }

    fn find(&self, item: &T) -> Option<usize>
    where
        T: PartialEq,
    {
        self.data
            .iter()
            .take(self.size)
            .position(|x| x.as_ref() == Some(item))
    }

    fn capacity(&self) -> usize {
        self.data.len()
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
}

#[cfg(test)]
mod tests {
    use super::UnsortedArray;
    use crate::arrays::{Array, ArrayError};

    #[test]
    fn new_array_has_zero_size_and_given_capacity() {
        let arr: UnsortedArray<i32> = UnsortedArray::new(5);
        assert_eq!(arr.size(), 0);
        assert_eq!(arr.capacity(), 5);
        assert!(arr.is_empty());
        assert!(!arr.is_full());
    }

    #[test]
    fn insert_success_until_full_then_error() {
        let mut arr = UnsortedArray::new(2);

        assert_eq!(arr.insert(10), Ok(()));
        assert_eq!(arr.insert(20), Ok(()));
        assert_eq!(arr.size(), 2);
        assert!(arr.is_full());

        let err = arr.insert(30).unwrap_err();
        assert!(matches!(err, ArrayError::Full));
    }

    #[test]
    fn delete_from_empty_returns_error() {
        let mut arr = UnsortedArray::<i32>::new(3);
        let err = arr.delete(0).unwrap_err();
        assert!(matches!(err, ArrayError::Empty));
    }

    #[test]
    fn delete_out_of_bounds_returns_error() {
        let mut arr = UnsortedArray::new(3);
        arr.insert(1).unwrap();
        let err = arr.delete(1).unwrap_err();
        assert!(matches!(err, ArrayError::IndexOutOfBounds));
    }

    #[test]
    fn delete_swaps_with_last_and_reduces_size() {
        let mut arr = UnsortedArray::new(5);
        arr.insert(10).unwrap(); // index 0
        arr.insert(20).unwrap(); // index 1
        arr.insert(30).unwrap(); // index 2

        // delete middle element (index 1)
        let deleted = arr.delete(1).unwrap();
        assert_eq!(deleted, 20);
        assert_eq!(arr.size(), 2);

        // now indices 0 and 1 must be Some(..), and one of them is 30
        let v0 = *arr.get(0).unwrap();
        let v1 = *arr.get(1).unwrap();
        assert!(matches!((v0, v1), (10, 30) | (30, 10)));
    }

    #[test]
    fn find_returns_correct_index_or_none() {
        let mut arr = UnsortedArray::new(4);
        arr.insert(5).unwrap();
        arr.insert(7).unwrap();
        arr.insert(9).unwrap();

        assert_eq!(arr.find(&5), Some(0));
        assert_eq!(arr.find(&7), Some(1));
        assert_eq!(arr.find(&9), Some(2));
        assert_eq!(arr.find(&42), None);
    }

    #[test]
    fn get_respects_bounds_and_size() {
        let mut arr = UnsortedArray::new(3);
        arr.insert(1).unwrap();
        arr.insert(2).unwrap();

        assert_eq!(arr.get(0), Some(&1));
        assert_eq!(arr.get(1), Some(&2));
        assert_eq!(arr.get(2), None); // index == size
    }

    #[test]
    fn traverse_applies_function_to_each_element_in_order() {
        let mut arr = UnsortedArray::new(4);
        arr.insert(1).unwrap();
        arr.insert(2).unwrap();
        arr.insert(3).unwrap();

        let mut acc = 0;
        arr.traverse(|x| {
            acc += *x;
            Ok(())
        })
        .unwrap();

        assert_eq!(acc, 1 + 2 + 3);
    }

    #[test]
    fn traverse_propagates_error_from_function() {
        let mut arr = UnsortedArray::new(3);
        arr.insert(1).unwrap();
        arr.insert(2).unwrap();

        let res = arr.traverse(|x| {
            if *x == 2 {
                return Err(ArrayError::Full);
            }
            Ok(())
        });

        assert_eq!(res, Err(ArrayError::Full));
    }
}