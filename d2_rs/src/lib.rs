// Rotate by L and R
// 0-99 and is a circle so 99+1 = 0. Similarly 5 and then L10 = 95
//
// The dial starts by pointing at 50.
// The actual password is the number of times the dial is left pointing at 0 after any rotation in the sequence.

use std::{
    fs::File,
    io::{BufReader, Read},
    ops::RangeInclusive,
};

fn get_ranges() -> Vec<RangeInclusive<u64>> {
    let file = File::open("./assets/input.txt").unwrap();
    let mut file = BufReader::new(file);
    let mut buf: String = String::new();
    let _ = file.read_to_string(&mut buf);
    let ranges = buf
        .split(',')
        .map(|f| {
            let parts = f.split('-').collect::<Vec<&str>>();
            println!("{:?}", parts);
            parts[0].parse::<u64>().unwrap()..=parts[1].parse::<u64>().unwrap()
        })
        .collect::<Vec<RangeInclusive<u64>>>();

    ranges
}

fn sum_invalid_values(ranges: Vec<RangeInclusive<u64>>) -> u64 {
    ranges
        .iter()
        .cloned()
        .flat_map(|f| f.collect::<Vec<u64>>())
        .filter(|&f| is_invalid(f))
        .inspect(|f| print!("{},", f))
        .fold(0u64, |acc, elem| acc + elem)
}

fn is_invalid(num: u64) -> bool {
    let len_digits = num.ilog10() + 1;
    let upper_half = num / 10u64.pow(len_digits / 2);
    let repeated = upper_half + upper_half * 10u64.pow(len_digits / 2);
    len_digits % 2 != 1 && num == repeated
}

#[cfg(test)]
mod tests {
    use crate::{get_ranges, sum_invalid_values};

    #[test]
    fn test_output() {
        let ranges = get_ranges();
        println!("\n-------------");
        let sum = sum_invalid_values(ranges);
        println!("\n-------------\n");
        assert_eq!(sum, 5398419778);
    }
}
