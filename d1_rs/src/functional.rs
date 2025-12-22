// Rotate by L and R
// 0-99 and is a circle so 99+1 = 0. Similarly 5 and then L10 = 95
//
// The dial starts by pointing at 50.
// The actual password is the number of times the dial is left pointing at 0 after any rotation in the sequence.

use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
};

struct AccResult {
    zero_count: u32,
    value: i32,
}

fn main() {
    let file = File::open("./assets/input.txt").unwrap();
    let file = BufReader::new(file);

    let result = file.lines().fold(
        AccResult {
            zero_count: 0,
            value: 50,
        },
        |acc, elem| {
            let code = elem.unwrap();
            let mut zero_count = acc.zero_count;
            let mut value = acc.value;

            let mut code_bytes = code.trim_end().bytes();
            let rotation_char = code_bytes.next().unwrap();
            let rotation = if rotation_char == b'R' { 1 } else { -1 };
            let num_bytes = code_bytes.take(code.len() - 1).collect::<Vec<u8>>();
            let num = str::from_utf8(&num_bytes).unwrap().parse::<i32>().unwrap();

            println!("Code: {}, {}, {}", code, rotation, num);
            value += rotation * num;
            value = value % 100;
            if value < 0 {
                value += 100;
            }
            if value == 0 {
                zero_count += 1;
            }

            AccResult {
                value: value,
                zero_count: zero_count,
            }
        },
    );

    println!("Zero count: {}", result.zero_count);
}
