use std::collections::HashMap;
use std::io;

use crate::third_task::custom_sorting;

pub fn show_department_personal (office: &Vec<String>, personal: &HashMap<String, Vec<String>>, task_num: u32) -> Vec<(String, Vec<String>)> {
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

    loop {
        if office.contains(&office_name.to_string()) {
            let mut employees: Vec<String> = match personal.get(&office_name).cloned() {
                Some(value) => value,
                None => {
                    println!("Для цього підрозділу співробітників не знайдено!");
                    break Vec::new();
                }
            };
            
            if custom_sorting::is_cyrillic_char(&employees) {
                custom_sorting::custom_sort(&mut employees);
            }
            else {
                employees.sort();
            }
            
            if task_num == 6 {
                employees.reverse();
            }

            let office_eployees: Vec<(String, Vec<String>)> = vec![(office_name, employees)];
            return office_eployees;
        }
        else {
            println!("Такого підрозділу не існує!");
            office_name.clear();
            break Vec::new();
        }
    }
}
