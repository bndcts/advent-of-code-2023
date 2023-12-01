use std::fs;

fn main() {
    first_puzzle();
}

fn first_puzzle() {
    let file_path = "src/input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let split_contents = contents.split("\n");
    let mut total: usize = 0;
    for line in split_contents {
        let mut first: char= 'a';
        let mut second: char = 'b';
        for c in line.chars() {
            if c.is_numeric() {
                if first == 'a' {
                    first = c;
                    second = c;
                } else {
                    second = c;
                }
            }
        }
        let mut res_string = String::new();
        res_string.push(first);
        res_string.push(second);
        let num: usize = res_string.parse().expect("Should have been able to parse");
        total += num;
    }
    println!("Total:\n{total}");
}
