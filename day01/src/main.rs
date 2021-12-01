use std::fs;
use itertools::Itertools;

type Data = Vec<u32>;

fn parse(fname: &str) -> Data {
    String::from_utf8(fs::read(fname).expect("missing file"))
        .unwrap()
        .lines()
        .map(|x| {
             x
                .parse()
                .unwrap()
            })
        .collect()
}

#[test]
fn test_part1(){
    assert_eq!(part1(parse("test0")), 7)
}

fn part1(data: Data) -> usize {
    data.into_iter()
        .tuple_windows()
        .filter(|(prev,next)| next>prev)
        .count()
}

#[test]
fn test_part2(){
    assert_eq!(part2(parse("test0")), 5)
}

fn part2(data: Data) -> usize {
    let windowed:Vec<u32> = data
        .into_iter()
        .tuple_windows::<(_,_,_)>()
        .map(|(x, y, z)| x + y + z)
        .collect();
    part1(windowed)
}

fn main() {
    println!("{:?}", part1(parse("input")));
    println!("{:?}", part2(parse("input")));
}
