use data_structures::arrays::{unsorted::UnsortedArray, Array};

fn main() {
    // Создаём неотсортированный массив целых чисел на 5 элементов
    let mut arr: UnsortedArray<i32> = UnsortedArray::new(5);

    println!("size = {}, capacity = {}", arr.size(), arr.capacity());

    // Вставляем несколько значений
    arr.insert(10).unwrap();
    arr.insert(20).unwrap();
    arr.insert(30).unwrap();

    println!("После вставки: size = {}", arr.size());
    for i in 0..arr.size() {
        println!("arr[{}] = {:?}", i, arr.get(i));
    }

    // Ищем элемент по значению
    if let Some(idx) = arr.find(&20) {
        println!("Найдено значение 20 по индексу {}", idx);
    } else {
        println!("Значение 20 не найдено");
    }

    // Удаляем элемент по индексу (обратите внимание, порядок не сохраняется)
    let deleted = arr.delete(1).unwrap();
    println!("Удалили элемент {} по индексу 1", deleted);
    println!("После удаления: size = {}", arr.size());

    for i in 0..arr.size() {
        println!("arr[{}] = {:?}", i, arr.get(i));
    }

    // Пробуем вставить до заполнения
    arr.insert(40).unwrap();
    arr.insert(50).unwrap();
    arr.insert(60).unwrap();
    println!("После вставки: size = {}", arr.size());
    for i in 0..arr.size() {
        println!("arr[{}] = {:?}", i, arr.get(i));
    }

    println!("Массив заполнен? {}", arr.is_full());

    // Попытка вставить сверх ёмкости
    match arr.insert(70) {
        Ok(()) => println!("Успешно вставили 70 (неожиданно)"),
        Err(e) => println!("Не удалось вставить 70: {:?}", e),
    }

    // Обход всех элементов с помощью traverse
    println!("Обход элементов через traverse:");
    let mut sum = 0;
    arr.traverse(|x| {
        println!("  элемент = {}", x);
        sum += *x;
        Ok(())
    })
    .unwrap();
    println!("Сумма всех элементов = {}", sum);
}
