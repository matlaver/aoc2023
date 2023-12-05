use std::collections::HashMap;

fn main() {
    let read_file = read_file("./src/input");
    
    let mut total: i64 = 0;
    for item in read_file.split_whitespace() {
        let num = find_first_last_numeric(item.to_string());

       let thenum: i64 = num.parse().unwrap();
        total += thenum;
    
    }
    println!("sum: {}", total); // should be 54208
}

// function to read text file
fn read_file(filename: &str) -> String {
    use std::fs;
    fs::read_to_string(filename).expect("Error reading file")
}

// function to find first and last numeric value in string
fn find_first_last_numeric(s: String) -> String {

    let mut textnumbers = HashMap::new();
    textnumbers.insert(String::from("one"), 1);
    textnumbers.insert(String::from("two"), 2);
    textnumbers.insert(String::from("three"), 3);
    textnumbers.insert(String::from("four"), 4);
    textnumbers.insert(String::from("five"), 5);
    textnumbers.insert(String::from("six"), 6);
    textnumbers.insert(String::from("seven"), 7);
    textnumbers.insert(String::from("eight"), 8);
    textnumbers.insert(String::from("nine"), 9);

    let mut first: String = "0".to_string();
    let mut first_index = 0;
    let mut last: String = "0".to_string();
    let mut last_index = 0;   

    let mut index = 0;

    for c in s.chars() {
        if c.is_numeric() {
            if first == "0" {
                first = c.to_string();
                first_index = s.find(c).unwrap();
            }
            last = c.to_string();
            last_index = index;
        }

        index += 1;
    }


    println!("first: {}", first);
    println!("last: {}", last);

    for (key, value) in &textnumbers {

        let v: Vec<_> = s.match_indices(key).collect();

        for (found_key, found_value) in v {          
            if first == "0".to_string() {
                first = value.to_string();
                first_index = index;  
            }

            if found_key < first_index {
                println!("found key < first_index: {}", found_key);
                first_index = found_key;
                first = value.to_string();             
            }

            if found_key >= last_index {
                last_index = found_key;
                last = value.to_string();               
            } 
        }
    }

    let result = first + &last;
    result
}


#[test]
fn test_single_ints() {
    assert_eq!(find_first_last_numeric("1".to_string()), "11");
    assert_eq!(find_first_last_numeric("2".to_string()), "22");
    assert_eq!(find_first_last_numeric("9".to_string()), "99"); 
    assert_eq!(find_first_last_numeric("xxxxx9".to_string()), "99"); 
}

#[test]
fn test_double_ints() {
    assert_eq!(find_first_last_numeric("12".to_string()), "12");
    assert_eq!(find_first_last_numeric("23".to_string()), "23");
    assert_eq!(find_first_last_numeric("99".to_string()), "99"); 
}

#[test]
fn test_strings() {
    assert_eq!(find_first_last_numeric("one".to_string()), "11");
    assert_eq!(find_first_last_numeric("twoone".to_string()), "21");
    assert_eq!(find_first_last_numeric("eightnineseven".to_string()), "87");
    assert_eq!(find_first_last_numeric("fivesevenfive".to_string()), "55");
}

#[test]
fn test_mix() {
    assert_eq!(find_first_last_numeric("2one".to_string()), "21");
    assert_eq!(find_first_last_numeric("nine8".to_string()), "98");  
    assert_eq!(find_first_last_numeric("five7sevenfive".to_string()), "55");  
}

#[test]
fn test_mix_with_padding() {
    assert_eq!(find_first_last_numeric("xxx2one".to_string()), "21");
    assert_eq!(find_first_last_numeric("xxthreecccone".to_string()), "31"); 
}

#[test]
fn test_overlapping_numbers() {
    assert_eq!(find_first_last_numeric("eightwo".to_string()), "82");
    assert_eq!(find_first_last_numeric("eighthree".to_string()), "83");
    assert_eq!(find_first_last_numeric("fivenine".to_string()), "59");
    assert_eq!(find_first_last_numeric("eighthree1".to_string()), "81");
}

