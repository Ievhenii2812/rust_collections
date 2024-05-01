use std::io;

pub mod average;
pub mod mediana_moda;

pub fn do_vector_operations() {
    //let my_numbers: Vec<i32> = vec![22, 55, 88, 22, 33, 77, 55, 22, 11, 77, 44];
    let mut my_numbers: Vec<i32> = Vec::new();

    println!("Введіть по одному на запит числа вектора:");

    let mut i = 1;
    while i <= 11 {
        println!("Позиція {}:", i);
        let mut number = String::new();

        io::stdin().read_line(&mut number)
               .expect("Неможливо прочитати строку.");

        let number: i32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => 0
        };

        my_numbers.push(number);

        i +=1;
    }

    use average::get_average;
    let average = get_average(&my_numbers);
    println!("Среднее значение = {}", average);

    mediana_moda::get_mediana(&my_numbers);

    mediana_moda::get_moda(&my_numbers);
}
