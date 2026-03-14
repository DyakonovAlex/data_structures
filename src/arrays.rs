pub mod unsorted;

#[derive(Debug, PartialEq)]
pub enum ArrayError {
    Full,
    Empty,
    IndexOutOfBounds,
}

pub trait Array<T> {
    /// Вставка элемента в массив
    fn insert(&mut self, item: T) -> Result<(), ArrayError>;

    /// Удаление элемента по индексу
    fn delete(&mut self, index: usize) -> Result<T, ArrayError>;

    /// Поиск элемента по значению
    fn find(&self, item: &T) -> Option<usize>
    where
        T: PartialEq;

    /// Получение элемента по индексу
    fn get(&self, index: usize) -> Option<&T>;

    /// Размер массива
    fn size(&self) -> usize;

    /// Емкость массива
    fn capacity(&self) -> usize;

    /// Проверка на пустоту
    fn is_empty(&self) -> bool {
        self.size() == 0
    }

    /// Проверка на заполненность
    fn is_full(&self) -> bool {
        self.size() == self.capacity()
    }

    /// Обход массива
    fn traverse(&self, mut f: impl FnMut(&T) -> Result<(), ArrayError>) -> Result<(), ArrayError> {
        for i in 0..self.size() {
            if let Some(item) = self.get(i) {
                f(item)?;
            } else {
                return Err(ArrayError::IndexOutOfBounds);
            }
        }
        Ok(())
    }
}
