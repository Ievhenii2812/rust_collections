use std::io;

mod first_task;
mod second_task;
mod third_task;
 
 fn main() {
    println!("Виберіть операцію, яку Ви хочете здійснити:");
    println!("1 - середнє значення, медіана та мода вектора");
    println!("2 - поросяча латинь");
    println!("3 - співробітники компанії по відділах");
    println!("Зробіть Ваш вибір:");

    let mut selected_operation = String::new();

    io::stdin().read_line(&mut selected_operation)
               .expect("Неможливо прочитати строку.");

    let selected_operation: u32 = match selected_operation.trim().parse() {
        Ok(num) => num,
        Err(_) => 0
    };

    match selected_operation {
        1 => first_task::do_vector_operations(),
        2 => second_task::pig_latin(),
        3 => third_task::organization(),
        _ => println!("Вам варто використовувати лише цифри від 1 до 3!"),
    }
}
