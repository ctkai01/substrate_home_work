pub fn check_sub_array(sub_array: &Vec<i32>, array: &Vec<i32>) -> bool {
    if sub_array.len() == 0 {
        return true;
    }
    if sub_array.len() > array.len() {
        return false;
    }

    let mut array_check_order: Vec<usize> = Vec::new();
    for &number in sub_array {
        let index = array.iter().position(|&e| e == number).unwrap_or(usize::MAX);
        if index == usize::MAX {
            return false
        }
        array_check_order.push(index);

        match array_check_order.iter().max() {
            Some(max) => {
                if max != &index {
                    return false
                }
            },
            None  => return false,
        }
       
    }
    true
}

