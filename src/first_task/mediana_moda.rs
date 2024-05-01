use std::collections::HashMap;


pub fn get_mediana(my_numbers: &Vec<i32>) {
    let mut my_numbers_clone: Vec<i32> = my_numbers.clone();
    my_numbers_clone.sort();

    let my_num_centre: usize = my_numbers_clone.len() / 2;
    let central_num = *my_numbers_clone.get(my_num_centre).unwrap_or(&-1);

    println!("Центральное значение вектора после сортировки (медиана): {}", central_num);
}

pub fn get_moda (my_numbers: &Vec<i32>) {
    let mut map = HashMap::new();

    // Считаем количество вхождений каждого элемента в вектор
    for &num in my_numbers {
        *map.entry(num).or_insert(0) += 1;
    }
    
    // Находим элемент с наибольшим количеством вхождений (моду)
    let mut mode = None;
    let mut max_count = 0;
    for (&num, &count) in &map {
        if count > max_count {
            mode = Some(num);
            max_count = count;
        }
    }
    
    if let Some(mode) = mode {
        println!("Мода {}: {} разів", mode, max_count);
    } else {
        println!("Мода не найдена");
    }
}