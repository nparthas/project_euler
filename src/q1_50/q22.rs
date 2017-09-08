use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

pub fn q22() -> i64 {

    let contents: String = {
        let path = Path::new("src/q1_50/TXTFiles/p022_names.txt");
        let display = path.display();

        let file = match File::open(&path) {

            Err(why) => panic!("Could not open {}: {}", display, why.description()),
            Ok(file) => file,

        };

        let mut buf_reader = BufReader::new(file);
        let mut read_contents = String::new();
        buf_reader.read_to_string(&mut read_contents).expect(
            "Error while reading file",
        );
        read_contents.replace("\"", "")
    };

    let mut contents_vec: Vec<&str> = contents.split(",").collect();
    insertion_sort(&mut contents_vec);

    let mut total: i64 = 0;
    let mut counter: i64 = 1;
    for i in contents_vec {

        total += counter * alphabetic_value(i);
        counter += 1;

    }

    return total;
}

fn insertion_sort(vec: &mut Vec<&str>) {
    let mut i: isize = 1;
    let mut key: &str;
    let mut j: isize;
    while i < vec.len() as isize {
        key = vec[i as usize];
        j = i - 1;

        while j >= 0 && vec[j as usize] > key {
            vec[(j + 1) as usize] = vec[j as usize];
            j -= 1;

        }
        vec[(j + 1) as usize] = key;
        i += 1;
    }


}

fn alphabetic_value(string: &str) -> i64 {

    let mut total: i64 = 0;
    for i in string.chars() {
        total += i as i64 % 32;
    }
    return total;
}
