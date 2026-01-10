use std::{
    fs::File,
    io::{BufRead, BufReader},
    usize,
};

use bevy::{
    color::LinearRgba,
    ecs::{component::Component, resource::Resource},
    math::Vec3,
    tasks::Task,
};
use rand::Rng;

#[derive(Clone, Copy)]
struct UFElem {
    parent: u16,
    size: usize,
}

type Point = (u32, u32, u32);

pub type TaskType = (usize, Vec<Point>, Vec<Circuit>, Option<u32>);

#[derive(Resource)]
pub struct ComputeTask(pub Task<TaskType>);

#[derive(Component, Clone, Debug)]
pub struct Circuit {
    pub color: LinearRgba,
    pub jboxes: Vec<Vec3>,
}

pub fn calculate(file_path: &str, limit: Option<usize>) -> TaskType {
    let file = File::open(file_path).unwrap();
    let mut buf = BufReader::new(file);
    let points = get_points(&mut buf);
    let mut d = Vec::with_capacity(points.len());
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            d.push((i as u16, j as u16, distance_euclidean(points[i], points[j])));
        }
    }
    d.sort_unstable_by_key(|p| p.2);
    let mut set = (0..points.len())
        .map(|i| UFElem {
            parent: i as u16,
            size: 1,
        })
        .collect::<Vec<UFElem>>();
    let limit = if let Some(limit) = limit {
        limit
    } else {
        usize::MAX
    };

    let mut last_pair = None;
    for &(i, j, _) in d.iter().take(limit) {
        if union(&mut set, i, j) == points.len() {
            last_pair = Some((i, j));
            break;
        }
    }
    let mut circuits = vec![Vec::<Vec3>::new(); points.len()];
    for i in 0..points.len() {
        circuits[find(&mut set, i as u16).parent as usize].push(Vec3::new(
            points[i].0 as f32,
            points[i].1 as f32,
            points[i].2 as f32,
        ));
    }
    circuits.sort_unstable_by_key(|c| c.len());
    circuits.reverse();

    let prod: usize = circuits.iter().take(3).map(|f| f.len()).product();
    let mut rng = rand::rng();
    let circuits = circuits
        .clone()
        .iter()
        .cloned()
        .map(|f| Circuit {
            color: LinearRgba::rgb(rng.random(), rng.random(), rng.random()),
            jboxes: f,
        })
        .collect();

    let last_pair_x_product = if let Some(last_pair) = last_pair {
        Some(points[last_pair.0 as usize].0 * points[last_pair.1 as usize].0)
    } else {
        None
    };
    (prod, points, circuits, last_pair_x_product)
}

fn get_points(buf: &mut BufReader<File>) -> Vec<Point> {
    buf.lines().map(|l| line_to_tuple3(&l.unwrap())).collect()
}

fn line_to_tuple3(s: &str) -> Point {
    let parts = s
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    (parts[0], parts[1], parts[2])
}

fn distance_euclidean(a: Point, b: Point) -> u64 {
    let dx = a.0.abs_diff(b.0) as u64;
    let dy = a.1.abs_diff(b.1) as u64;
    let dz = a.2.abs_diff(b.2) as u64;

    dx * dx + dy * dy + dz * dz
}

fn find(set: &mut Vec<UFElem>, x: u16) -> UFElem {
    let xu = x as usize;
    if x == set[xu].parent {
        set[xu]
    } else {
        set[xu] = find(set, set[xu].parent);
        set[xu]
    }
}

fn union(set: &mut Vec<UFElem>, x: u16, y: u16) -> usize {
    let x = find(set, x);
    let y = find(set, y);

    if x.parent == y.parent {
        x.size
    } else {
        let combined_size = x.size + y.size;
        set[x.parent as usize].parent = y.parent as u16;
        set[y.parent as usize].size += x.size;
        combined_size
    }
}

#[cfg(test)]
mod tests {
    use crate::circuits::calculate;

    #[test]
    fn test_sample() {
        let (product, _, _, _) = calculate("./assets/sample.txt", Some(10));
        assert_eq!(40, product);
    }

    #[test]
    fn test_personal_case() {
        let (product, _, _, _) = calculate("./assets/personal.txt", Some(1000));
        assert_eq!(164475, product);
    }

    #[test]
    fn test_sample_pt2() {
        let (_, _, _, last_pair_x_product) = calculate("./assets/sample.txt", None);
        assert_eq!(Some(25272), last_pair_x_product);
    }

    #[test]
    fn test_personal_pt2() {
        let (_, _, _, last_pair_x_product) = calculate("./assets/personal.txt", None);
        assert_eq!(Some(169521198), last_pair_x_product);
    }
}
