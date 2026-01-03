use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn count_values_within_ranges(ranges: Vec<(u64, u64)>, values: Vec<u64>) -> u16 {
    let mut starts = ranges.iter().map(|f| f.0).collect::<Vec<u64>>();
    let starts = &mut starts[..];
    starts.sort_unstable();
    let mut ends = ranges.iter().map(|f| f.1).collect::<Vec<u64>>();
    let ends = &mut ends[..];
    ends.sort_unstable();
    let sorted_values = &mut values.clone()[..];
    sorted_values.sort_unstable();

    println!("Starts: {:?}", starts);
    println!();
    println!("Ends: {:?}", ends);

    let len_ranges = ranges.len();
    let mut out = 0u16;
    // Possible optimization would be to start the indexes from the lowest start that the last iteration reached because
    // We already have everything sorted anyways, so there's no need to go look from the start.
    for value in values {
        let mut last_start_idx = 0;
        let mut last_end_idx = 0;

        for i in 0..len_ranges {
            if value >= starts[i] {
                last_start_idx = i + 1;
                println!("  Matching start at index {}: {}", i, starts[i]);
            } else {
                println!("  No more matching starts after index {}: {}", i, starts[i]);
                break;
            }
        }

        for i in 0..len_ranges {
            if value > ends[i] {
                last_end_idx = i + 1;
                println!("  Matching end at index {}: {}", i, ends[i]);
            } else {
                println!("  No more matching ends after index {}: {}", i, ends[i]);
                break;
            }
        }

        let num_ranges = (last_start_idx as i32) - (last_end_idx as i32);
        if num_ranges > 0 && num_ranges < len_ranges as i32 {
            // This one will count in how many ranges the value is contained within
            // out += num_ranges as u16;

            // For the first challenge we just need to know if it is within any range
            out += 1;
        }

        println!(
            "Value: {}, last_start_idx: {}, last_end_idx: {}, num_ranges: {}",
            value, last_start_idx, last_end_idx, num_ranges
        );
    }

    out
}

fn read_parts(input: File) -> (Vec<(u64, u64)>, Vec<u64>) {
    let file = BufReader::new(input);

    let mut lines = file.lines();
    let ranges: Vec<(u64, u64)> = lines
        .by_ref()
        .map_while(Result::ok)
        .take_while(|l| !l.is_empty())
        .map(split_range)
        .collect();
    // Newline is consumed by the take_while at this point so no skip mechanism is needed
    let values: Vec<u64> = lines
        .filter_map(Result::ok)
        .map(|l| l.parse::<u64>().unwrap())
        .collect();

    (ranges, values)
}

fn split_range(line: String) -> (u64, u64) {
    let parts = line.split("-").collect::<Vec<&str>>();
    (
        parts[0].parse::<u64>().unwrap(),
        parts[1].parse::<u64>().unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_case() {
        let file = File::open("./assets/sample.txt").unwrap();

        let (ranges, values) = read_parts(file);
        println!("Ranges: {:?}", &ranges);
        println!();
        println!("Values: {:?}", &values);
        println!();

        let total = count_values_within_ranges(ranges, values);
        println!("Total: {}", total);
        assert_eq!(3, total);
    }

    // $ time cargo test
    // cargo test  0.07s user 0.03s system 89% cpu 0.111 total
    #[test]
    fn test_personal_case() {
        let file = File::open("./assets/personal.txt").unwrap();

        let (ranges, values) = read_parts(file);
        println!("Ranges: {:?}", &ranges);
        println!();
        println!("Values: {:?}", &values);

        let total = count_values_within_ranges(ranges, values);
        assert_eq!(888u16, total);
    }
}
