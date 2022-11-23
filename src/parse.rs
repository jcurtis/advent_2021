use std::fs;
use std::path;

pub fn parse_i32(s: &str) -> i32 {
    s.parse().unwrap()
}

pub fn input_to_integer_vec(input: &str) -> Vec<i32> {
    let lines = input.trim().lines().map(parse_i32);
    Vec::from_iter(lines)
}

pub fn read_file(file_name: &str) -> String {
    let path = path::Path::new(file_name);
    let contents = fs::read_to_string(path).expect("Error reading file");
    contents
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse_i32() {
        assert_eq!(super::parse_i32("32"), 32);
        assert_eq!(super::parse_i32("-32"), -32);
    }

    #[test]
    fn input_to_integer_vec() {
        let file_name = "test_input/input_to_integer_vec.txt";
        let contents = super::read_file(file_name);

        let result = super::input_to_integer_vec(&contents);

        assert_eq!(result.len(), 10);
        assert_eq!(
            result,
            vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]
        );
    }
}
