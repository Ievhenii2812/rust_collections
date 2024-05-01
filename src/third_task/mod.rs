use std::collections::HashMap;

pub mod task;
pub mod office;
pub mod personal;
pub mod all_personal;
pub mod department_personal;
pub mod custom_sorting;

pub fn organization() {
    let mut task_num: u32;
    let mut office: Vec<String> = Vec::new();
    let mut personal: HashMap<String, Vec<String>> = HashMap::new();
    
    loop {
        task_num = task::select_task();

        match task_num {
            1 => {
                println!("Ваш вибір: {}", task_num);
                let office_name: String = office::add_office();
                office.push(office_name);
                continue;
            },
            2 => {
                println!("Ваш вибір: {}", task_num);
                let (person_office, person_name) = personal::add_person(&office);
                if !person_office.is_empty() {
                    personal.entry(person_office).or_insert(Vec::new()).push(person_name);
                }
                continue;
            },
            3 => {
                println!("Ваш вибір: {}", task_num);
                let all_personal_asc: Vec<(String, Vec<String>)> = all_personal::show_all_personal_asc(&personal);
                println!("Перелік співробітників підприємства: {:?}", all_personal_asc);
                continue;
            },
            4 => {
                println!("Ваш вибір: {}", task_num);
                let all_personal_desc: Vec<(String, Vec<String>)> = all_personal::show_all_personal_desc(&personal);
                println!("Перелік співробітників підприємства: {:?}", all_personal_desc);
                continue;
            },
            5 => {
                println!("Ваш вибір: {}", task_num);
                let department_employees_asc: Vec<(String, Vec<String>)> = department_personal::show_department_personal(&office, &personal, task_num);
                if !department_employees_asc.is_empty() {
                    println!("Співробітники відділу: {:?}", department_employees_asc);
                }
                continue;
            },
            6 => {
                println!("Ваш вибір: {}", task_num);
                let department_employees_asc: Vec<(String, Vec<String>)> = department_personal::show_department_personal(&office, &personal, task_num);
                if !department_employees_asc.is_empty() {
                    println!("Співробітники відділу: {:?}", department_employees_asc);
                }
                continue;
            },
            _ => {
                println!("Закінчення виконання програми. Дякую за користування!");
                break;
            },
        }
    }
}
