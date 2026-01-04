use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn merge_ranges(ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut starts = ranges.iter().map(|f| f.0).collect::<Vec<u64>>();
    let starts = &mut starts[..];
    starts.sort_unstable();
    let mut ends = ranges.iter().map(|f| f.1).collect::<Vec<u64>>();
    let ends = &mut ends[..];
    ends.sort_unstable();

    let len_ranges = ranges.len();

    // Vec has more capacity allocated than necessary but it's fine in this case. We are sacrificing a bit of
    // memory for performance juice
    let mut ranges = Vec::with_capacity(len_ranges);
    let mut si = 0;
    loop {
        if si >= len_ranges {
            break;
        }

        let mut pi = 0;
        let siv = starts[si];

        for i in 1..(len_ranges - si) {
            println!(
                "Comparing start {} with end {}",
                starts[si + i],
                ends[si + pi]
            );
            if starts[si + i] > ends[si + pi] {
                println!(
                    "  No more overlap found: start {} > end {}",
                    starts[si + i],
                    ends[si + pi]
                );
                break;
            }
            println!(
                "  Overlap found: start {} <= end {}",
                starts[si + i],
                ends[si + pi]
            );
            pi += 1;
        }

        ranges.push((siv, ends[si + pi]));
        si += if pi > 0 { pi + 1 } else { 1 };
    }

    ranges
}

///
/// The idea is to just sort start and end and values and count how many starts <= x <= end
/// We then substract the index of the last start we found from the index of the last end we found, like so:
/// let num_ranges = (last_start_idx as i32) - (last_end_idx as i32);
///
/// If num_ranges is negative or zero, that means that the last range-end that we found had an index higher than the
/// last range-start that we found. Basically the value is outside of any range.
///
/// If the num_ranges is a positive integer, then its value equals to how many ranges contain this value!
/// We are of course skipping that because the challenge doesn't require it but it was a nice learning experience on how
/// to achieve that specific task too.
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

        let cloned_ranges = ranges.clone();

        let total = count_values_within_ranges(ranges, values);
        println!("Total: {}", total);
        assert_eq!(3, total);

        // Challenge 2

        let merged_ranges = merge_ranges(cloned_ranges);
        let sum_ranges = merged_ranges
            .iter()
            .fold(0, |acc, elem| acc + (elem.1 - elem.0) + 1);
        println!("Merged ranges: {:?}", merged_ranges);
        println!("Sum of ranges: {}", sum_ranges);
        assert_eq!(14, sum_ranges);
    }

    // The release binary will run this in less than 1ms with a warm cache
    #[test]
    fn test_personal_case() {
        let file = File::open("./assets/personal.txt").unwrap();

        let (ranges, values) = read_parts(file);
        println!("Ranges: {:?}", &ranges);
        println!();
        println!("Values: {:?}", &values);

        let merged_ranges = merge_ranges(ranges.clone());
        let sum_ranges = merged_ranges
            .iter()
            .fold(0, |acc, elem| acc + (elem.1 - elem.0) + 1);
        println!("Merged ranges: {:?}", merged_ranges);
        println!("Sum of ranges: {}", sum_ranges);
        assert_eq!(344378119285354, sum_ranges);

        let total = count_values_within_ranges(merged_ranges, values);
        assert_eq!(888u16, total);
    }
}
