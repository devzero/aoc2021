use std::fs;

enum Instr {
    Up(usize),
    Down(usize),
    Forward(usize),
}

fn to_instr(line: &str) -> Instr {
    let tokens: Vec<&str> = line.split_ascii_whitespace().collect();
    match (tokens[0], tokens[1].parse::<usize>().unwrap()) {
        ("down", num) => Instr::Down(num),
        ("up", num) => Instr::Up(num),
        ("forward", num) => Instr::Forward(num),
        _ => panic!(),
    }
}

fn parse(fname: &str) -> Vec<Instr> {
    String::from_utf8(fs::read(fname).expect("missing file"))
        .unwrap()
        .lines()
        .map(to_instr)
        .collect()
}

#[test]
fn test_part1() {
    assert_eq!(part1(parse("test0")), 150)
}

fn part1(instrs: Vec<Instr>) -> usize {
    let (h, d) = instrs
        .iter()
        .fold((0usize, 0usize), |(horiz, depth), instr| match instr {
            Instr::Up(x) => (horiz, depth - x),
            Instr::Down(x) => (horiz, depth + x),
            Instr::Forward(x) => (horiz + x, depth),
        });
    h * d
}

#[test]
fn test_part2() {
    assert_eq!(part2(parse("test0")), 900)
}

fn part2(instrs: Vec<Instr>) -> usize {
    let (h, d, _) =
        instrs.iter().fold(
            (0usize, 0usize, 0usize),
            |(horiz, depth, aim), instr| match instr {
                Instr::Up(x) => (horiz, depth, aim - x),
                Instr::Down(x) => (horiz, depth, aim + x),
                Instr::Forward(x) => (horiz + x, depth + (aim * x), aim),
            },
        );
    h * d
}

fn main() {
    println!("{:?}", part1(parse("input")));
    println!("{:?}", part2(parse("input")));
}
