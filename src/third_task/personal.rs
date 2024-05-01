use std::io;

pub fn add_person(office: &Vec<String>) -> (String, String) {
    println!("Виберіть підрозділ, до якого требі додати співробітника");
    let mut number: usize = 1;
    for i in office {
        println!("{} - {}", number, i);
        number += 1;
    }

    let mut office_code = String::new();
    let mut office_name: String;
    match io::stdin().read_line(&mut office_code) {
        Ok(_) => {
            let selected_office_code: usize = match office_code.trim().parse() {
                Ok(num) => num,
                Err(_) => 0,
            };

            if selected_office_code > 0 && selected_office_code <= office.len() {
                office_name = office[selected_office_code - 1].clone();//clone тому що нам потрібне значення, а у нас є посилання
            }
            else {
                office_name = "Такого відділу не існує".to_string();
            }
        }
        Err(_) => {
            office_name = "Такого відділу не існує".to_string();
        }
    }

    let mut person: String = String::new();

    loop {
        if office.contains(&office_name.to_string()) {
            person = person_data();
            return (office_name.to_string(), person);
        }
        else {
            println!("Такого підрозділу не існує!");
            office_name.clear();
            person.clear();
            break (String::new(), String::new());
        }
    }
}

fn person_data() -> String {
    println!("Додайте дані співробітника");

    let mut person_full_name: String = String::new();

    loop {
        match io::stdin().read_line(&mut person_full_name) {
            Ok(_) => {
                let trimmed_person_name = person_full_name.trim();
                if !trimmed_person_name.is_empty() {
                    break trimmed_person_name.to_string();
                } else {
                    println!("Введена порожня строка!");
                    person_full_name.clear();
                }
            }
            Err(error) => {
                eprintln!("Помилка при спробі прочитати строку: {}. Спробуйте ще раз!", error);
                continue;
            }
        }
    }
}