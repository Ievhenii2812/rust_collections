use std::collections::HashMap;

pub fn custom_sort(strings: &mut Vec<String>) {
    // Таблиця відповідності літер до числових кодів
    let mut letter_to_code: HashMap<char, i32> = HashMap::new();
    letter_to_code.insert(' ', 1);
    letter_to_code.insert('А', 2);
    letter_to_code.insert('а', 3);
    letter_to_code.insert('Б', 4);
    letter_to_code.insert('б', 5);
    letter_to_code.insert('В', 6);
    letter_to_code.insert('в', 7);
    letter_to_code.insert('Г', 8);
    letter_to_code.insert('г', 9);
    letter_to_code.insert('Д', 10);
    letter_to_code.insert('д', 11);
    letter_to_code.insert('Е', 12);
    letter_to_code.insert('е', 13);
    letter_to_code.insert('Є', 14);
    letter_to_code.insert('є', 15);
    letter_to_code.insert('Ё', 16);
    letter_to_code.insert('ё', 17);
    letter_to_code.insert('Ж', 18);
    letter_to_code.insert('ж', 19);
    letter_to_code.insert('З', 20);
    letter_to_code.insert('з', 21);
    letter_to_code.insert('И', 22);
    letter_to_code.insert('и', 23);
    letter_to_code.insert('І', 24);
    letter_to_code.insert('і', 25);
    letter_to_code.insert('Ї', 26);
    letter_to_code.insert('ї', 27);
    letter_to_code.insert('Й', 28);
    letter_to_code.insert('й', 29);
    letter_to_code.insert('К', 30);
    letter_to_code.insert('к', 31);
    letter_to_code.insert('Л', 32);
    letter_to_code.insert('л', 33);
    letter_to_code.insert('М', 34);
    letter_to_code.insert('м', 35);
    letter_to_code.insert('Н', 36);
    letter_to_code.insert('н', 37);
    letter_to_code.insert('О', 38);
    letter_to_code.insert('о', 39);
    letter_to_code.insert('П', 40);
    letter_to_code.insert('п', 41);
    letter_to_code.insert('Р', 42);
    letter_to_code.insert('р', 43);
    letter_to_code.insert('С', 44);
    letter_to_code.insert('с', 45);
    letter_to_code.insert('Т', 46);
    letter_to_code.insert('т', 47);
    letter_to_code.insert('У', 48);
    letter_to_code.insert('у', 49);
    letter_to_code.insert('Ф', 50);
    letter_to_code.insert('ф', 51);
    letter_to_code.insert('Х', 52);
    letter_to_code.insert('х', 53);
    letter_to_code.insert('Ц', 54);
    letter_to_code.insert('ц', 55);
    letter_to_code.insert('Ч', 56);
    letter_to_code.insert('ч', 57);
    letter_to_code.insert('Ш', 58);
    letter_to_code.insert('ш', 59);
    letter_to_code.insert('Щ', 60);
    letter_to_code.insert('щ', 61);
    letter_to_code.insert('Ъ', 62);
    letter_to_code.insert('ъ', 63);
    letter_to_code.insert('Ы', 64);
    letter_to_code.insert('ы', 65);
    letter_to_code.insert('Ь', 66);
    letter_to_code.insert('ь', 67);
    letter_to_code.insert('Э', 68);
    letter_to_code.insert('э', 69);
    letter_to_code.insert('Ю', 70);
    letter_to_code.insert('ю', 71);
    letter_to_code.insert('Я', 72);
    letter_to_code.insert('я', 73);
    
    // Функція для отримання числового представлення рядка
    let string_to_numeric = |s: &String| -> Vec<i32> {
    s.chars()
    .map(|c| *letter_to_code.get(&c).unwrap_or(&0))
    .collect()
    };
    
    // Сортування вектора рядків за числовим представленням
    strings.sort_by_cached_key(string_to_numeric);
}

pub fn is_cyrillic_char(strings: &Vec<String>) -> bool {
    strings[0].chars().all(|c| c.is_cyrillic() || c.is_numeric() || c.is_punctuation())
}

trait CharExtensions {
    fn is_cyrillic(self) -> bool;
    fn is_punctuation(self) -> bool;
    }
    
    impl CharExtensions for char {
    fn is_cyrillic(self) -> bool {
    ('а'..='я').contains(&self) || ('А'..='Я').contains(&self)
    }
    
    fn is_punctuation(self) -> bool {
    ['.', ',', '!', '?', ';', ':', '-', '(', ')', '[', ']', '{', '}', '"', '\''].contains(&self)
    }
    }