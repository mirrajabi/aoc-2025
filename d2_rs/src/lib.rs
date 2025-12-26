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
        .filter(|&f| !is_valid(f))
        .inspect(|f| print!("{},", f))
        .fold(0u64, |acc, elem| acc + elem)
}

// Doesn't cover single digits because it's not clear if those are valid or invalid but the dataset doesn't contain them
//  so it's safe to keep like this
fn is_valid(num: u64) -> bool {
    let s: String = num.to_string();
    let s = s.as_bytes();
    let len = s.len();
    let prefix_list = get_prefix_list(s, len);
    !is_pure_repeats(prefix_list, len)
}

// From: https://cp-algorithms.com/string/prefix-function.html#implementation
fn get_prefix_list(s: &[u8], n: usize) -> Vec<u8> {
    let mut pi = vec![0u8; n];
    for i in 1..n {
        let mut j = pi[i - 1] as usize;
        while j > 0 && s[i] != s[j] {
            j = pi[j - 1] as usize;
        }

        if s[i] == s[j] {
            j += 1;
        }
        pi[i] = j as u8
    }

    pi
}

fn is_pure_repeats(prefix_list: Vec<u8>, strlen: usize) -> bool {
    let lsp = prefix_list[prefix_list.len() - 1];
    lsp >= ((strlen as u8) + 1) / 2 && lsp % ((strlen as u8) - lsp) == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output() {
        let ranges = get_ranges();
        println!("\n-------------");
        let sum = sum_invalid_values(ranges);
        println!("\n-------------\n");
        assert_eq!(sum, 15704845910);
    }

    #[test]
    fn test_prefix_list() {
        println!("1: {:?}", get_prefix_list("1".as_bytes(), 1));
        println!("11: {:?}", get_prefix_list("11".as_bytes(), 2));
        println!("758758: {:?}", get_prefix_list("758758".as_bytes(), 6));
        assert_eq!(is_pure_repeats(get_prefix_list("758758".as_bytes(), 6), 6), true);
        println!("7587587: {:?}", get_prefix_list("7587587".as_bytes(), 7));
        assert_eq!(is_pure_repeats(get_prefix_list("7587587".as_bytes(), 7), 7), false);
        println!("1221221: {:?}", get_prefix_list("1221221".as_bytes(), 7));
        println!("758875887: {:?}", get_prefix_list("758875887".as_bytes(), 9));
        println!("118856118851: {:?}", get_prefix_list("118856118851".as_bytes(), 12));
        println!("101: {:?}", get_prefix_list("101".as_bytes(), 3));
        println!("2121212121: {:?}", get_prefix_list("2121212121".as_bytes(), 10));
        assert_eq!(is_pure_repeats(get_prefix_list("2121212121".as_bytes(), 10), 10), true);
    }
}
