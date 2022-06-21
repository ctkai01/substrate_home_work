#[derive(Debug)]
struct Fibonacci {
    a: u32,
    b: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.a + self.b;
        let item_a = self.a;
        self.a = self.a + self.b;
        self.b = item_a;
        Some(result)
    }
}

fn fibonacci_numbers() -> Fibonacci {
    Fibonacci { a: 0, b: 1 }
}

pub fn print_fibonacci() {
    for number in fibonacci_numbers() {
        println!("{}", number);
    }
}
