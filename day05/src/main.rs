use std::cmp::max;
use std::collections::HashMap;

type Point = (i32, i32);
type Line = (Point, Point);
type LineList = Vec<Line>;

fn vec2tup<T>(v: Vec<T>) -> (T, T)
where
    T: Copy,
{
    (v[0], v[1])
}

fn parse(fname: &str) -> LineList {
    let input = std::fs::read_to_string(fname).unwrap();
    input
        .lines()
        .map(|line| {
            vec2tup(
                line.split(" -> ")
                    .map(|point_str| {
                        vec2tup(
                            point_str
                                .split(",")
                                .map(|numstr| numstr.parse::<i32>().unwrap())
                                .collect::<Vec<i32>>(),
                        )
                    })
                    .collect(),
            )
        })
        .collect()
}

fn is_straight(((x0, y0), (x1, y1)): &Line) -> bool {
    (x0 == x1) | (y0 == y1)
}

fn interpolate(((x0, y0), (x1, y1)): Line) -> Vec<Point> {
    let mut points: Vec<Point> = vec![];
    let diff_x = x1 - x0;
    let diff_y = y1 - y0;
    let steps = max(diff_x.abs(), diff_y.abs());
    points.push((x0, y0));
    if steps != 0 {
        //case of p0==p1, div by zero
        let delta_x = diff_x / steps;
        let delta_y = diff_y / steps;
        for i in 1..=steps {
            let new_x = x0 + i * delta_x;
            let new_y = y0 + i * delta_y;
            points.push((new_x, new_y));
        }
    }
    points
}

fn part1(line_list: LineList) -> usize {
    part2(line_list.into_iter().filter(is_straight).collect())
}

fn part2(line_list: LineList) -> usize {
    line_list
        .into_iter()
        .map(|l| interpolate(l))
        .flatten()
        .fold(HashMap::new(), |mut hmap: HashMap<Point, usize>, p| {
            *hmap.entry(p).or_insert(0) += 1;
            hmap
        })
        .into_values()
        .filter(|&c| c > 1)
        .count()
}

#[test]
fn test_part1() {
    assert_eq!(part1(parse("test0")), 5)
}

#[test]
fn test_part2() {
    assert_eq!(part2(parse("test0")), 12)
}

fn main() {
    println!("{:?}", part1(parse("input")));
    println!("{:?}", part2(parse("input")));
}
