fn main() {
    let read_file = read_file("./src/input");
    
    let mut total = 0;
    for item in read_file.split_whitespace() {
        let num = find_first_last_numeric(item.to_string());

       let thenum: i32 = num.parse().unwrap();
        total += thenum;
    
    }
    println!("sum: {}", total);
}

// function to read text file
fn read_file(filename: &str) -> String {
    use std::fs;
    fs::read_to_string(filename).expect("Error reading file")
}

// function to find first and last numeric value in string
fn find_first_last_numeric(s: String) -> String {
    let mut first: char = '0';
    let mut last: char = '0';   
    for c in s.chars() {
        if c.is_numeric() {
            if first == '0' {
                first = c;
            }
            last = c;
        }
    }
    
    let mut result = first.to_string();
    result.push(last);
    result
}
