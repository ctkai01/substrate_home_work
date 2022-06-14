pub fn reverse_vector(original_vector: &mut Vec<u8>) -> Vec<u8> {
    let mut reverse: Vec<u8> = Vec::new();
    
    loop {
        if original_vector.len() == 0 {
            break;
        }
        let value = original_vector.pop().unwrap();
        reverse.push(value)
    }
    reverse
}
