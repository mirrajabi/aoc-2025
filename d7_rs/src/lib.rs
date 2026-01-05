const CHAR_START: u8 = b'S';
const CHAR_SPACE: u8 = b'.';
const CHAR_BEAM: u8 = b'|';
const CHAR_SPLITTER: u8 = b'^';
const CHAR_SPLITTED: u8 = b'+';

fn calc(grid: &str) -> u32 {
    let mut source_grid = grid.lines().flat_map(|f| f.bytes()).collect::<Vec<u8>>();

    let len = source_grid.len();
    let w = grid.lines().next().unwrap().len();
    let h = len / w;

    let print_fn = |state: &[u8]| {
        let printable_state = state
            .chunks(w)
            .map(|row| String::from_utf8_lossy(row))
            .collect::<Vec<_>>()
            .join("\n");
        println!("{}", printable_state);
        println!("--------------------------------");
    };
    let index_of = |row: isize, col: isize| {
        if row >= 0 && row < h as isize && col >= 0 && col < w as isize {
            Some(row as usize * w + col as usize)
        } else {
            None
        }
    };

    println!("w: {}, h: {}", w, h);
    print_fn(&source_grid);

    // Separate iterator for first line to reduce branches on the hot path
    let mut sx = None;
    for col in 0..w {
        let val = source_grid[index_of(0, col as isize).unwrap()];
        if val != CHAR_START {
            continue;
        }

        sx = Some(col);
        let idx_down = index_of(0 + 1, col as isize).unwrap();
        let down = source_grid[idx_down];
        if down == CHAR_SPLITTER {
            panic!("We don't support an immediate splitter under S yet.");
        }

        source_grid[idx_down] = CHAR_BEAM;
        break;
    }

    if let None = sx {
        panic!("Well I'll be damned!");
    }
    let split_count = traverse(&mut source_grid, &index_of, &print_fn, 1, sx.unwrap());
    print_fn(&source_grid);
    println!("Split count: {:?}", split_count);
    split_count
}

fn traverse(
    state: &mut [u8],
    index_of_fn: &impl Fn(isize, isize) -> Option<usize>,
    print_fn: &impl Fn(&[u8]),
    current_y: usize,
    current_x: usize,
) -> u32 {
    // print_fn(&state);
    let idx_down = index_of_fn(current_y as isize + 1, current_x as isize);
    if let None = idx_down {
        return 0;
    }

    let idx_down = idx_down.unwrap();
    match state[idx_down] {
        CHAR_SPACE => {
            state[idx_down] = CHAR_BEAM;
            traverse(state, index_of_fn, print_fn, current_y + 1, current_x)
        }
        CHAR_SPLITTER => {
            state[idx_down] = CHAR_SPLITTED;
            let mut count = 1;
            if let Some(idx_se) = index_of_fn(current_y as isize + 2, current_x as isize + 1) {
                state[idx_se] = CHAR_BEAM;

                count += traverse(state, index_of_fn, print_fn, current_y + 2, current_x + 1);
            }
            if let Some(idx_sw) = index_of_fn(current_y as isize + 2, current_x as isize - 1) {
                state[idx_sw] = CHAR_BEAM;
                count += traverse(state, index_of_fn, print_fn, current_y + 2, current_x - 1);
            }
            count
        }
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{BufReader, Read},
    };

    use super::*;

    #[test]
    fn test_sample_case() {
        let file = File::open("./assets/sample.txt").unwrap();
        let mut buf = BufReader::new(file);
        let mut grid = String::new();
        if let Ok(read_bytes) = buf.read_to_string(&mut grid) {
            println!("Read {} bytes", read_bytes);
            let split_count = calc(&grid);

            assert_eq!(21, split_count);
        } else {
            println!("Explosion!");
        }
    }

    #[test]
    fn test_personal_case() {
        let file = File::open("./assets/personal.txt").unwrap();
        let mut buf = BufReader::new(file);
        let mut grid = String::new();
        buf.read_to_string(&mut grid);
        let split_count = calc(&grid);

        assert_eq!(1541, split_count);
    }
}
