type Input = Vec<i64>;

fn parse(fname: &str) -> Input {
    std::fs::read_to_string(fname)
        .unwrap()
        .split(',')
        .map(|s| s.trim().parse::<i64>().unwrap())
        .collect()
}

fn dist(l: &Vec<i64>, i: i64) -> i64 {
    l.into_iter().map(|&x| (x - i).abs()).sum()
}

fn median(l: &Vec<i64>) -> i64 {
    let mut lp = l.clone();
    lp.sort();
    lp[lp.len() / 2]
}

fn part1(data: Input) -> i64 {
    dist(&data, median(&data))
}

fn weird_dist(l: &Vec<i64>, i: i64) -> i64 {
    l.into_iter()
        .map(|&x| (x - i).abs())
        .map(|y| y * (y + 1) / 2)
        .sum()
}
fn part2(data: Input) -> i64 {
    let minval = *data.iter().min().unwrap();
    let maxval = *data.iter().max().unwrap();
    (minval..maxval)
        .into_iter()
        .map(|i| weird_dist(&data, i))
        .min()
        .unwrap()
}

#[test]
fn test_part1() {
    assert_eq!(part1(parse("test0")), 37)
}

#[test]
fn test_part2() {
    assert_eq!(part2(parse("test0")), 168)
}

fn main() {
    println!("{:?}", part1(parse("input")));
    println!("{:?}", part2(parse("input")));
}
