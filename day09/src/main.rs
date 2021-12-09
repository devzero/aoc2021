#[derive(Debug)]
struct HeatMap {
    x: usize,
    y: usize,
    d: Vec<u32>,
}

fn parse(fname: &str) -> HeatMap {
    let mut x = 0;
    let mut y = 0;
    let mut d = Vec::new();
    for line in std::fs::read_to_string(fname).unwrap().lines() {
        x = x.max(line.len());
        y += 1;
        let mut dd: Vec<u32> = line.chars().map(|n| n.to_digit(10).unwrap()).collect();
        d.append(&mut dd);
    }
    HeatMap { d: d, x: x, y: y }
}

fn is_low_point(m: &HeatMap, x: usize, y: usize) -> bool {
    let val = get_val(m, x as isize, y as isize).unwrap();
    [
        (x as isize - 1, y as isize),
        (x as isize, y as isize - 1),
        (x as isize + 1, y as isize),
        (x as isize, y as isize + 1),
    ]
    .into_iter()
    .filter_map(|(cx, cy)| get_val(&m, cx, cy))
    .all(|p| val < p)
}

fn get_val(m: &HeatMap, x: isize, y: isize) -> Option<u32> {
    if (x < 0) | (y < 0) | ((x as usize) >= m.x) | ((y as usize) >= m.y) {
        return None;
    };
    Some(m.d[(y as usize) * m.x + (x as usize)])
}

fn part1(m: HeatMap) -> usize {
    let mut risk: usize = 0;
    for j in 0..m.y {
        for i in 0..m.x {
            if is_low_point(&m, i, j) {
                let val = get_val(&m, i as isize, j as isize).unwrap();
                risk += val as usize + 1;
            }
        }
    }
    risk
}

fn find_basin(m: &HeatMap, startx: isize, starty: isize) -> usize {
    let mut seen: std::collections::HashSet<(isize, isize)> = Default::default();
    let mut basin: std::collections::HashSet<(isize, isize)> = Default::default();
    let mut todo: std::collections::HashSet<(isize, isize)> = Default::default();
    todo.insert((startx, starty));
    while !todo.is_empty() {
        let (x, y) = todo.clone().into_iter().next().unwrap();
        todo.remove(&(x, y));
        let val_opt = get_val(m, x, y);
        seen.insert((x, y));
        if let None = val_opt {
            continue;
        }
        let val = val_opt.unwrap();
        if val == 9 {
            continue;
        }
        basin.insert((x, y));
        for p in [
            (x as isize - 1, y as isize),
            (x as isize, y as isize - 1),
            (x as isize + 1, y as isize),
            (x as isize, y as isize + 1),
        ] {
            if !seen.contains(&p) {
                todo.insert(p);
            }
        }
    }
    basin.len()
}

fn part2(m: HeatMap) -> usize {
    let mut basin_nums: Vec<usize> = Vec::new();
    for j in 0..m.y {
        for i in 0..m.x {
            if is_low_point(&m, i, j) {
                let found = find_basin(&m, i as isize, j as isize);
                basin_nums.push(found);
            }
        }
    }
    basin_nums.sort();
    basin_nums.into_iter().rev().take(3).product()
}
#[test]
fn test_part1() {
    assert_eq!(part1(parse("test0")), 15)
}

#[test]
fn test_part2() {
    assert_eq!(part2(parse("test0")), 1134)
}

fn main() {
    println!("{:?}", part1(parse("input")));
    println!("{:?}", part2(parse("input")));
}
