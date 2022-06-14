use std::io;
mod exercise_1;
mod exercise_2;
mod exercise_4;

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
                    return length;
                }
            }
            Err(_) => println!("Please enter a number"),
        }
    }
}

fn flow_exercise_1() {
    let x = exercise_1::change_value(10, &mut 20);
    println!("X: {}", x)
}

fn flow_exercise_2() {
    let mut count: u32 = 1;
    let mut num: u64 = 1;
    let mut primes: Vec<u64> = Vec::new();
    primes.push(2);

    while count < 10 {
        num += 2;
        if exercise_2::vector_is_prime(num, &primes) {
            count += 1;
            primes.push(num);
        }
    }
    println!("{:?}", primes);
}

fn flow_exercise_3() {
        let mut values = vec![10, 11, 12];
    let v = &mut values;

    let mut max = 0;
    
    for n in &mut *v {
        max = std::cmp::max(max, *n);
    }

    println!("max is {}", max);
    println!("Converting to percentages of maximum value...");
    
    for n in v {
        *n = 100 * (*n) / max;
    }
    println!("values: {:#?}", values);
}

fn flow_exercise_4() {
    let mut original_vector: Vec<u8> = vec![1,2,3,4,5];
    
    println!("Vector before reverse: {:?}", original_vector);
    let original_vector = exercise_4::reverse_vector(&mut original_vector);
    println!("Vector after reverse: {:?}", original_vector);
}

fn main_menu() {
    fn show() {
        println!("");
        println!("==== Exercise ====");
        println!("1. Exercise 1");
        println!("2. Exercise 2");
        println!("3. Exercise 3");
        println!("4. Exercise 4");
    }

    loop {
        show();
        let input = get_number_input_larger_zero();

        match input {
            1 => flow_exercise_1(),
            2 => flow_exercise_2(),
            3 => flow_exercise_3(),
            4 => flow_exercise_4(),
            _ => break,
        }
    }
}

fn main() {
    main_menu()
}
