use data_structures::arrays::{sorted::SortedArray, Array};

fn main() {
    // Создаём отсортированный массив целых чисел на 6 элементов
    let mut arr: SortedArray<i32> = SortedArray::new(6);

    // Вставляем в произвольном порядке — внутри всегда будет сортировка
    for x in [5, 1, 4, 2, 3] {
        arr.insert(x).unwrap();
    }

    println!("size = {}, capacity = {}", arr.size(), arr.capacity());
    println!("Элементы (всегда отсортированы):");
    for i in 0..arr.size() {
        println!("arr[{}] = {}", i, arr.get(i).unwrap());
    }

    // Поиск (использует свойство сортировки)
    println!("find(4) = {:?}", arr.find(&4));
    println!("find(42) = {:?}", arr.find(&42));

    // Удаление по индексу: порядок сохраняется, элементы сдвигаются влево
    let deleted = arr.delete(2).unwrap();
    println!("Удалили элемент {} по индексу 2", deleted);

    println!("После удаления:");
    for i in 0..arr.size() {
        println!("arr[{}] = {}", i, arr.get(i).unwrap());
    }

    // Обход всех элементов через traverse
    let mut sum = 0;
    arr.traverse(|x| {
        sum += *x;
        Ok(())
    })
    .unwrap();
    println!("Сумма всех элементов = {}", sum);
}

