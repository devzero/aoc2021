type Board = [u8; 100];

fn parse(fname: &str) -> Board {
    std::fs::read_to_string(fname)
        .unwrap()
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .flatten()
        .collect::<Vec<u8>>()
        .try_into()
        .unwrap()
}

fn adjacents(i: usize) -> Vec<usize> {
    let x = i % 10;
    let y = i / 10;
    let mut adjs = Vec::new();
    if (y > 0) && (x > 0) {
        adjs.push(i - 11);
    }
    if y > 0 {
        adjs.push(i - 10);
    }
    if (y > 0) && (x < 9) {
        adjs.push(i - 9)
    }
    if x > 0 {
        adjs.push(i - 1);
    }
    if x < 9 {
        adjs.push(i + 1);
    }
    if (y < 9) && (x > 0) {
        adjs.push(i + 9);
    }
    if y < 9 {
        adjs.push(i + 10);
    }
    if (y < 9) && (x < 9) {
        adjs.push(i + 11);
    }
    adjs
}

fn step(b: Board) -> (Board, Vec<usize>) {
    let mut flashes = Vec::new();
    let mut next = [0u8; 100];
    for i in 0..100 {
        next[i] = b[i] + 1;
    }
    while next
        .iter()
        .enumerate()
        .any(|(i, &x)| (x > 9) && (!flashes.contains(&i)))
    {
        for (i, x) in next.into_iter().enumerate() {
            if (x > 9) && !flashes.contains(&i) {
                for j in adjacents(i) {
                    next[j] += 1;
                }
                flashes.push(i);
            }
        }
    }

    for (i, x) in next.into_iter().enumerate() {
        if x > 9 {
            next[i] = 0;
        }
    }
    (next, flashes)
}

fn part1(b: Board) -> usize {
    let mut board = b;
    let mut seen_flashes = 0;
    for _ in 0..100 {
        let (x, flashes) = step(board);
        seen_flashes += flashes.len();
        board = x;
    }
    seen_flashes
}

fn part2(b: Board) -> usize {
    let mut steps = 0;
    let mut flashes_this_step = 0;
    let mut board = b;
    while flashes_this_step < 100 {
        let (x, flashes) = step(board);
        flashes_this_step = flashes.len();
        board = x;
        steps += 1;
    }
    steps
}

#[test]
fn test_part1() {
    assert_eq!(part1(parse("test0")), 1656)
}

#[test]
fn test_part2() {
    assert_eq!(part2(parse("test0")), 195)
}

fn main() {
    println!("{:?}", part1(parse("input")));
    println!("{:?}", part2(parse("input")));
}
