type Fish = [u128; 9];

fn parse(fname: &str) -> Fish {
    std::fs::read_to_string(fname)
        .unwrap()
        .split(",")
        .into_iter()
        .map(|s| s.trim().parse::<usize>().unwrap())
        .fold([0u128; 9], |mut acc, val| {
            acc[val] += 1;
            acc
        })
}

fn step_day(f: Fish) -> Fish {
    [f[1], f[2], f[3], f[4], f[5], f[6], f[7] + f[0], f[8], f[0]]
}

fn step_days(days: usize, fish: Fish) -> Fish {
    (0..days).fold(fish, |acc, _| step_day(acc))
}
fn part1(fish: Fish) -> u128 {
    step_days(80, fish).iter().sum()
}

fn part2(fish: Fish) -> u128 {
    step_days(256, fish).iter().sum()
}

#[test]
fn test_part1() {
    assert_eq!(part1(parse("test0")), 5934)
}

#[test]
fn test_part2() {
    assert_eq!(part2(parse("test0")), 26984457539)
}

fn main() {
    println!("{:?}", part1(parse("input")));
    println!("{:?}", part2(parse("input")));
}
