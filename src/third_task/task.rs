use std::io;

pub fn select_task () -> u32 {
    println!("Виберіть задачу для виконання:");
    println!("1 - Внести дані про новий відділ");
    println!("2 - Внести дані нового співробітника");
    println!("3 - Отримати дані всіх співробітників, відсортований за алфавітом від початку");
    println!("4 - Отримати дані всіх співробітників, відсортований за алфавітом з кінця");
    println!("5 - Отримати дані співробітників підрозділу, відсортований за алфавітом від початку");
    println!("6 - Отримати дані співробітників підрозділу, відсортований за алфавітом з кінця");
    println!("7 - Завершити програму");

    let mut selected_task = String::new();

    loop {
        match io::stdin().read_line(&mut selected_task) {
            Ok(_) => {
                match selected_task.trim() {
                    "1" | "2" | "3" | "4" | "5" | "6" | "7"
                    => {
                        let selected_operation: u32 = match selected_task.trim().parse() {
                            Ok(num) => num,
                            Err(_) => 0
                        };
                        break selected_operation;
                        //return selected_operation;
                    }
                    _ => {
                        eprintln!("Виберіть одну цифру від 1 до 7!");
                        selected_task.clear();
                    }
                }
            },
            Err(error) => {
                eprintln!("Помилка при спробі прочитати строку: {}. Спробуйте ще раз!", error);
                continue;
            }
        }
    }
}