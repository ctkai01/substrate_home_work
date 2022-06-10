use std::{io, fs};

mod sub_array;
mod count_word;

fn get_input() -> String {
    let mut buffer = String::from("");

    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please enter your data again");
    }

    buffer.trim().to_owned()
}

fn get_number_input_larger_zero() -> i32 {
    loop {
        let number_parsed: Result<i32, _> = get_input().parse();

        match number_parsed {
            Ok(length) => {
                if length < 1 {
                    println!("Please enter a number larger than 0")
                } else {
                    return length
                }
            },
            Err(_) => println!("Please enter a number")
        }
    }
}

fn get_number_input() -> i32 {
    println!("Please enter number: ");
    loop {
        let number_parsed: Result<i32, _> = get_input().parse();

        match number_parsed {
            Ok(length) => {
                if length < 1 {
                    println!("Please enter a number larger than 0")
                } else {
                    return length
                }
            },
            Err(_) => println!("Please enter a number")
        }
    }
}

fn get_array_from_input() -> Vec<i32> {
    let len_array = get_number_input_larger_zero();
    let mut array: Vec<i32> = Vec::new();

    for _ in 0..len_array {
        let value = get_number_input();
        array.push(value);
    }

    array
}

fn flow_check_sub_array() {
    println!("Press length for array A: ");
    let array_a: Vec<i32> = get_array_from_input();

    println!("Press length for array B: ");
    let array_b: Vec<i32> = get_array_from_input();
    
    let check_sub_array = sub_array::check_sub_array(&array_b, &array_a);

    if check_sub_array {
        println!("Array {:?} is sub array in array {:?}", array_b, array_a);
    } else {
        println!("Array {:?} isn't sub array in array {:?}", array_b, array_a);
    }
}   

fn count_word_normal(input_given: &str) {
    println!("Press input search: ");
    let input_find = get_input();

    let count_word_match = count_word::count_words(&input_find, &input_given, count_word::SearchType::Normal);

    println!("String {:?} match {} count", input_find, count_word_match);
}

fn count_word_regex(input_given: &str) {
    println!("Press input search: ");
    let input_find = get_input();

    let count_word_match = count_word::count_words(&input_find, &input_given, count_word::SearchType::Regex);

    println!("String {:?} match regex {} count", input_find, count_word_match);

}


fn flow_count_word() {

    println!("1. Search normal");
    println!("2. Search regex");

    let string_given = match fs::read_to_string("src/input_string.txt") {
        Result::Ok(value) => value,
        Result::Err(error) => panic!("{}", error),
    };
    loop {
        let input = get_input();

        match input.as_str() {
            "1" => count_word_normal(&string_given),
            "2" => count_word_regex(&string_given),
            _ => break 
        }
    }
}   

fn main_menu() {
    fn show() {
        println!("");
        println!("==== Exercise ====");
        println!("1. Check array B whether is sub array in array A");
        println!("2. Count word appear in given string")
    }

    loop {
        show();
        let input = get_input();

        match input.as_str() {
            "1" => flow_check_sub_array(),
            "2" => flow_count_word(),
            _ => break 
        }
    }
}

fn main() {
    main_menu()
}
