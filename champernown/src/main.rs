fn main() {
    println!("Hello, world!");
}

fn champ_test(n: u64) -> i32 {

    let n_string: String = n.to_string();
    let mut string_index: usize = 0;
    let mut current_num: usize = 0;

    while string_index < n_string.len() {

        current_num += 1;

        let current_num_string = current_num.to_string();
        let num_length = current_num.to_string().len();
        let candidate = &n_string[string_index..string_index+num_length];

        if current_num_string != candidate {
            return -1;
        }

        string_index += num_length;
    }
    return current_num as i32;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(12, champ_test(123456789101112));
    }
}
