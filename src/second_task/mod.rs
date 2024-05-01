use std::io;

pub fn pig_latin() {
    println!("Введіть слово англійською, німецькою або українською:");

    let mut word = String::new();

    loop {
        match io::stdin().read_line(&mut word) {
            Ok(_) => break,
            Err(error) => {
                eprintln!("Помилка при спробі прочитати строку: {}. Спробуйте ще раз!", error);
                continue;
            }
        }
    }

    word = word.trim().to_string();

    if let Some(first_char) = word.chars().next() {
        // Перевіряємо результат виклику to_lowercase()
        match first_char.to_lowercase().next() {
            Some(first_char_lower) => {
                // Обробляємо перший символ
                match first_char_lower {
                    'а' | 'е' | 'ё' | 'и' | 'о' | 'у' | 'ы' | 'э' | 'ю' | 'я' |
                    'a' | 'e' | 'i' | 'o' | 'u' |
                    'ä' | 'ö' | 'ü' => {
                        let new_word = format!("{}-hay", &word);
                        println!("{}", new_word);
                    }
                    _ => {
                        let rest = &word[first_char.len_utf8()..];
                        let new_word = format!("{}-{}ay", rest, first_char);
                        println!("{}", new_word);
                    }
                }
            },
            None => {
                // Обробляємо випадок, коли символ не може бути переведений у нижній регістр
                eprintln!("Неможливо виконати операцію");
            }
        }
    } else {
        // Обробляємо випадок, коли слово порожнє або встановлюємо значення за замовчуванням
        eprintln!("Стрічка порожня");
    }
}