type Fish = [u128; 9];

fn parse(fname: &str) -> Fish {
    let input = std::fs::read_to_string(fname).unwrap();
    let mut input_fish: Fish = [0u128; 9];
    input
        .split(",")
        .into_iter()
        .map(|s| s.trim().parse::<usize>().unwrap())
        .for_each(|n| input_fish[n] += 1);
    input_fish
}

fn step_day(f: Fish) -> Fish {
    let mut new_fish = [0u128; 9];
    for (i, &val) in f.iter().enumerate() {
        if i == 0 {
            new_fish[8] = val;
            new_fish[6] = val;
        } else {
            new_fish[i - 1] += val;
        }
    }
    new_fish
}

fn part1(data: Fish) -> u128 {
    let mut fish = data;
    for _ in 0..80 {
        fish = step_day(fish);
    }
    fish.iter().sum()
}

fn part2(data: Fish) -> u128 {
    let mut fish = data;
    for _ in 0..256 {
        fish = step_day(fish);
    }
    fish.iter().sum()
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
