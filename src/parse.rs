use std::fs;
use std::path;

pub fn parse_file_integers(file_name: &str) -> Vec<i32> {
    let path = path::Path::new(file_name);

    // println!("Reading file {}", path.display());
    let contents = fs::read_to_string(path).expect("Error reading file");
    // println!("Contents: {}", &contents);

    let contents = contents.trim();
    fn parse_i32(s: &str) -> i32 {
        s.parse().unwrap()
    }
    let res = contents.lines().map(parse_i32);
    Vec::from_iter(res)
}

mod tests {
    #[test]
    fn parse_file_integers() {
        let file_name = "test_input/parse_file_integers.txt";
        let result = super::parse_file_integers(file_name);

        assert_eq!(result.len(), 10);
        assert_eq!(
            result,
            vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]
        );
    }
}
