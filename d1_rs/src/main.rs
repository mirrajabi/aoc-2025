// Rotate by L and R
// 0-99 and is a circle so 99+1 = 0. Similarly 5 and then L10 = 95
//
// The dial starts by pointing at 50.
// The actual password is the number of times the dial is left pointing at 0 after any rotation in the sequence.

use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
};

fn main() {
    let file = File::open("./assets/input.txt").unwrap();
    let mut file = BufReader::new(file);

    let start = 50;
    let mut value = start;
    let mut zero_count = 0;

    let mut buf = Default::default();
    while let Ok(count) = file.read_line(&mut buf)
        && count != 0
    {
        let code = &buf[0..count - 1];

        let mut code_bytes = code.bytes();
        let rotation_char = code_bytes.next().unwrap();
        let rotation = if rotation_char == b'R' { 1 } else { -1 };
        let num_bytes = code_bytes.take(count - 1).collect::<Vec<u8>>();
        let num = str::from_utf8(&num_bytes)
            .unwrap()
            .parse::<i32>()
            .unwrap();

        value += rotation * num;
        value = value % 100;
        if value < 0 {
            value += 100;
        }
        if value == 0 {
            zero_count += 1;
        }
        println!("{}, {}, Rotation: {}, {:?}, ", &code, &value, &rotation, &num);
        
        buf.clear();
    }
    println!("Zero count: {}", zero_count);
}
