use std::cmp::max;

// The idea is:
// We keep a vector of size N where N is how many digits are we interested in. So if
// we want the highest number with 2 digits then N=2.
//
// We iterate through the digits of each line, comparing the digit with all of the digits in the vector
// where it has the posibility to match. If the new digit is higher than the most significant possible digit
// then we replace the old with the new and set everything on the right side of it to 1 (1 is the lowest value in
// this challenge). If the digit isn't bigger than the most significant possible digit, we test it against the other digits
// on the right side.
// Edge cases include:
// we have to make sure that if a digit with a value higher than our current digits exists, it can
// only replace the digit that allows other digits fitting in our N digits!
// As an example: given 235329, at the point that we are iterating on the 9, our state is: [5, 3].
// Since there are no more digits in front of 9, we can only see 9 as a candidate for 3 and not for the 5.
#[tracing::instrument]
fn get_max(line: &str, n: usize) -> u64 {
    let row = line
        .trim()
        .as_bytes()
        .iter()
        .map(|f| f - 0x30)
        .collect::<Vec<u8>>();

    let row_len = row.len();

    if n > row_len {
        panic!("You can't expect a larger number to be find than the full input");
    }

    let mut l: Vec<u8> = vec![1u8; n];

    println!("Initial values: {:?}", l);

    for (index, digit) in row.iter().enumerate() {
        let digit = *digit;
        println!("-----------------------------");
        if digit == 1 {
            // 1 is the lowest possible digit in this challenge. It can't help us much.
            println!("Skipping 1 at index: {}", index);
            continue;
        }

        let mut j = max(n as isize - row_len as isize + index as isize, 0isize) as usize;
        println!(
            "Processing digit: {} at index: {}, starting j at: {}",
            digit, index, j
        );
        println!("l is: {:?}", l);
        loop {
            if j > index {
                println!("Breaking out of the loop. j > index isn't a valid position possibility.");
                break;
            }
            let current = l[j as usize];

            println!(
                "Comparing digit: {} with current: {} at l[{}]",
                digit, current, j
            );

            // Is digit larger than the most significant possible digit?
            // Then this + next digits make the highest we are aware of so far
            if digit > current {
                l[j] = row[index];
                println!("Setting: {:?}, l[{}] = row[{}]", l, index, index);

                for x in j + 1..n {
                    l[x] = 1;
                    println!("Setting: {:?}, l[{}] = 0", l, x);
                }
                break;
            } else {
                // If not, try the less significant digits
                j += 1;
                println!("Incrementing j to: {}", j);
            }

            if j >= n {
                println!("Breaking out of loop, j: {}", j);
                break;
            }
        }
    }
    let out = l.iter().fold((n as i32 - 1, 0), |acc, elem| {
        (
            acc.0 - 1,
            acc.1 + (*elem as u64) * (10_u64.pow(acc.0 as u32)),
        )
    });

    println!("Out: {:?}, {:?}", out.1, l);
    out.1
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs::File,
        io::{BufRead, BufReader},
        thread::sleep,
        time::Duration,
    };

    #[test]
    fn test_sample_case_2_digits() {
        let sample = "987654321111111
811111111111119
234234234234278
818181911112111";
        let sum = sample
            .lines()
            .map(|f| get_max(f, 2))
            .reduce(|acc, elem| acc + elem)
            .unwrap();
        assert_eq!(sum, 357);
    }

    #[test]
    fn test_sample_case_3_digits() {
        let sample = "987654321111111
811111111111119
234234234234278
818181911112111";
        let sum = sample
            .lines()
            .map(|f| get_max(f, 3))
            .reduce(|acc, elem| acc + elem)
            .unwrap();
        assert_eq!(sum, 3205);
    }

    #[test]
    fn test_sample_case_12_digits() {
        let sample = "987654321111111
811111111111119
234234234234278
818181911112111";
        let sum = sample
            .lines()
            .map(|f| get_max(f, 12))
            .reduce(|acc, elem| acc + elem)
            .unwrap();
        assert_eq!(sum, 3121910778619);
    }

    #[test]
    fn test_personal_case_2_digits() {
        let file = File::open("./assets/input.txt").unwrap();
        let file = BufReader::new(file);
        let sum = file
            .lines()
            .map(|f| get_max(&f.unwrap(), 2))
            .reduce(|acc, elem| acc + elem)
            .unwrap();

        assert_eq!(sum, 17493);
    }

    #[test]
    fn test_personal_case_2_digits_with_more_logging() {
        let file = File::open("./assets/input.txt").unwrap();
        let file = BufReader::new(file);
        let out = file
            .lines()
            .map(|f| get_max(&f.unwrap(), 2))
            .collect::<Vec<u64>>();

        sleep(Duration::from_secs(2));
        let file = File::open("./assets/input.txt").unwrap();
        let file = BufReader::new(file);
        let mut x = 0;
        file.lines().enumerate().for_each(|(index, line)| {
            println!("{}: {}", out[index], line.unwrap());
            x += out[index];
        });
        println!("X: {}", x);
    }

    #[test]
    fn test_personal_case_12_digits() {
        let file = File::open("./assets/input.txt").unwrap();
        let file = BufReader::new(file);
        let sum = file
            .lines()
            .map(|f| get_max(&f.unwrap(), 12))
            .reduce(|acc, elem| acc + elem)
            .unwrap();

        assert_eq!(sum, 173685428989126);
    }

    #[test]
    fn test_lines() {
        let max = get_max(
            "2411323321122342222312224225222113222113323212322221243612222112223322233231224121422335412222222422",
            2,
        );
        assert_eq!(max, 65);

        let max = get_max(
            "4374393634627266376474276644376666365551634337854472353256224441642396253632315522324245257365668356",
            2,
        );
        assert_eq!(max, 99);

        let max = get_max(
            "4666466462666765355556775566656666646455636655645365555344466454434466567568947453654466565466634464",
            2,
        );
        assert_eq!(max, 97);

        let max = get_max(
            "2844453127333213377123132313622231332413444742524422222142322232242322121722323231122332232323411143",
            2,
        );
        assert_eq!(max, 87);

        let max = get_max(
            "2844453127333213377123132313622231332413444742524422222142322232242322121722323231122332232323411143",
            3,
        );
        assert_eq!(max, 877);

        let max = get_max(
            "2844453127333213377123132313622231332413444742524422222142322232242322121722323231122332232323411143",
            4,
        );
        assert_eq!(max, 8777);

        let max = get_max(
            "2844453127333213377123132313622231332413444742524422222142322232242322121722323231122332232323411143",
            12,
        );
        assert_eq!(max, 877777411143)
    }

    #[test]
    fn test_line_5() {
        // // With 2 digits
        let max = get_max("987654321111111", 2);
        assert_eq!(max, 98);
        let max = get_max("811111111111119", 2);
        assert_eq!(max, 89);
        let max = get_max("234234234234278", 2);
        assert_eq!(max, 78);
        let max = get_max("818181911112111", 2);
        assert_eq!(max, 92);

        // // With 3 digits
        let max = get_max("987654321111111", 3);
        assert_eq!(max, 987);
        let max = get_max("811111111111119", 3);
        assert_eq!(max, 819);
        let max = get_max("234234234234278", 3);
        assert_eq!(max, 478);
        let max = get_max("818181911112111", 3);
        assert_eq!(max, 921);

        // // With 12 digits
        let max = get_max("987654321111111", 12);
        assert_eq!(max, 987654321111);
        let max = get_max("811111111111119", 12);
        assert_eq!(max, 811111111119);
        let max = get_max("234234234234278", 12);
        assert_eq!(max, 434234234278);
        let max = get_max("818181911112111", 12);
        assert_eq!(max, 888911112111);
    }

    #[test]
    fn test_one_optimizations() {
        let max = get_max("818181911112111", 12);
        assert_eq!(max, 888911112111);
    }
}
