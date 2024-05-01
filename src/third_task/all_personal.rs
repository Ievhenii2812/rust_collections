use std::collections::HashMap;
use crate::third_task::custom_sorting;

pub fn show_all_personal_asc (personal: &HashMap<String, Vec<String>>) -> Vec<(String, Vec<String>)> {
    let mut all_employees = personal.clone();
    // Сортуємо всі вектори у HashMap
    for (_, employees) in all_employees.iter_mut() {
        if custom_sorting::is_cyrillic_char(&employees) {
            custom_sorting::custom_sort(employees);
        }
        else {
            employees.sort();
        }
    }

    return all_employees.into_iter().collect();
}

pub fn show_all_personal_desc (personal: &HashMap<String, Vec<String>>) -> Vec<(String, Vec<String>)> {
    let mut all_employees = personal.clone();

    for (_, employees) in all_employees.iter_mut() {
        if custom_sorting::is_cyrillic_char(&employees) {
            custom_sorting::custom_sort(employees);
        }
        else {
            employees.sort();
        }
        employees.reverse(); // Зміна порядку значень у векторі у зворотньому порядку
    }

    return all_employees.into_iter().collect();
}
