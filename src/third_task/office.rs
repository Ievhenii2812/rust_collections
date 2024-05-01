use std::io;

pub fn add_office() -> String {
    println!("Введіть назву підрозділу");

    let mut office_name: String = String::new();

    loop {
        match io::stdin().read_line(&mut office_name) {
            Ok(_) => {
                let trimmed_office_name = office_name.trim();
                if !trimmed_office_name.is_empty() {
                    return trimmed_office_name.to_string();
                } else {
                    println!("Введена порожня строка!");
                    office_name.clear();
                }
            }
            Err(error) => {
                eprintln!("Помилка при спробі прочитати строку: {}. Спробуйте ще раз!", error);
                continue;
            }
        }
    }
}