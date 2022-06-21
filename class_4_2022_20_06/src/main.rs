use std::io;
mod fibonacci;
mod life_time;

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


fn main_menu() {
    fn show() {
        println!("");
        println!("==== Exercise ====");
        println!("1. Fibonacci");
        println!("2. Fix Life time");
    }

    loop {
        show();
        let input = get_number_input_larger_zero();

        match input{
            1 => fibonacci::print_fibonacci(),
            2 => life_time::print_vector(),
            _ => break 
        }
    }
}


fn main() {
    main_menu()
}