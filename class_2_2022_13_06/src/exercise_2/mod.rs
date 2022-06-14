
pub fn vector_is_prime(num: u64, p: &Vec<u64>) -> bool {
    for i in p {
        if num > *i && num % i == 0 {
            return false;
        }
    }

    true
}
