use std::{
    collections::{HashSet, VecDeque},
    rc::Rc,
};

const INDICATOR_BIT_LEN: usize = 32;
type Indicator = (usize, u32);

#[derive(Clone, Debug)]
struct LineParts {
    indicator: Indicator,
    buttons: Vec<Vec<u32>>,
    joltage_requirements: Vec<u32>,
}

#[derive(Clone, Debug)]
struct Node {
    parent: Option<Rc<Node>>,
    button_idx: Option<usize>,
    state: u32,
}

fn split_line(line: &str) -> LineParts {
    let indicator: Indicator = line[1..].split("]").collect::<Vec<&str>>()[0]
        .as_bytes()
        .iter()
        .map(|&c| if c == b'#' { 1 } else { 0 })
        .fold((0, 0u32), |(i, val), bit| {
            (i + 1, val | (bit << (INDICATOR_BIT_LEN - i - 1)))
        });

    let buttons = line[indicator.0 + 3..].split("{").collect::<Vec<&str>>()[0]
        .trim()
        .split(' ')
        .map(|p| {
            p[1..p.len() - 1]
                .split(',')
                .map(|f| f.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let joltage_requirements = line[..line.len() - 1].split("{").collect::<Vec<&str>>()[1]
        .split(',')
        .map(|c| c.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    LineParts {
        indicator,
        buttons,
        joltage_requirements,
    }
}

fn find_fewest_presses(indicator: u32, buttons: &Vec<Vec<u32>>) -> Option<Rc<Node>> {
    let root = Rc::new(Node {
        parent: None,
        button_idx: None,
        state: 0,
    });

    let mut visited = HashSet::new();
    visited.insert(root.state);
    let mut to_explore = VecDeque::new();
    to_explore.push_back(root);
    while let Some(parent) = to_explore.pop_front() {
        let mut valid_buttons = buttons
            .into_iter()
            .cloned()
            .map(|b| {
                let score = get_score(parent.state, &b);
                (b, score)
            })
            .collect::<Vec<(Vec<u32>, i32)>>();
        valid_buttons.sort_by_key(|f| -f.1);
        // println!("Sorted buttons: {:?}", &valid_buttons);

        for (idx, b) in valid_buttons.iter().enumerate() {
            let mut derived_state = parent.state;
            for wire in &b.0 {
                derived_state ^= 1 << (31 - wire);
            }

            if visited.contains(&derived_state) {
                continue;
            }

            // println!("Derived state for b={:?} is {:032b}", &b, derived_state);
            let child = Rc::new(Node {
                parent: Some(Rc::clone(&parent)),
                button_idx: Some(idx),
                state: derived_state,
            });

            if derived_state == indicator {
                return Some(child);
            }

            visited.insert(derived_state);
            to_explore.push_back(child);
        }
    }
    None
}

fn get_score(state: u32, buttons: &Vec<u32>) -> i32 {
    let mut positive = 0usize;
    let mut negative = 0usize;
    for wire in buttons {
        // println!(
        //     "wire: {}, (1 >> wire): {:032b}, (state << wire): {:032b}, and {:032b}",
        //     wire,
        //     1 << (31 - wire),
        //     (1 << (31 - wire) & state),
        //     (1 << (31 - wire) & state) >> (31 - wire)
        // );
        if (1 << (31 - wire) & state) >> (31 - wire) == 1 {
            positive += 1;
        } else {
            negative += 1;
        }
    }
    let total = positive as i32 - negative as i32;
    // println!(
    //     "At state {:032b}, the score for {:?} is {}; pos: {}, neg: {}",
    //     state, buttons, total, positive, negative
    // );

    total
}

fn get_depth(node: Rc<Node>) -> usize {
    let mut depth = 0;
    let mut current = node;
    // println!("Buttons for this phase:");
    while let Some(parent) = &current.parent {
        // println!("  {:?}", current.button);
        depth += 1;
        current = Rc::clone(parent);
    }
    depth
}

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    use super::*;

    #[test]
    #[ignore]
    fn test_sample_case() {
        let file = File::open("./assets/sample.txt").unwrap();
        let buf_reader = BufReader::new(file);
        let sum: usize = buf_reader
            .lines()
            .map(|line| split_line(&line.unwrap()))
            .inspect(|parts| {
                println!("indicator:");
                println!("  len: {}", parts.indicator.0);
                println!("  val: {:032b}", parts.indicator.1);
                println!("buttons: {:?}", parts.buttons);
                println!("joltage_requirements: {:?}", parts.joltage_requirements);
            })
            .map(|p| find_fewest_presses(p.indicator.1, &p.buttons))
            .filter_map(|f| f)
            .map(|f| get_depth(f))
            .sum();

        assert_eq!(7, sum);
    }

    #[test]
    fn test_personal_case() {
        let file = File::open("./assets/personal.txt").unwrap();
        let buf_reader = BufReader::new(file);
        let sum: usize = buf_reader
            .lines()
            .map(|line| split_line(&line.unwrap()))
            .map(|p| find_fewest_presses(p.indicator.1, &p.buttons))
            .filter_map(|f| f)
            .map(|f| get_depth(f))
            .sum();

        assert_eq!(7, sum);
    }
}
