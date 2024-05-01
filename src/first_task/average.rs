pub fn get_average(my_numbers: &Vec<i32>) -> i32 {
    let my_num_length: i32 = my_numbers.len().try_into().unwrap();
    
    let sum: i32 = my_numbers.iter().sum();

    sum / my_num_length
}
