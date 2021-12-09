type Digit = [bool; 7];
struct Input {
    training: [Digit; 10],
    testing: [Digit; 4],
}

type Inputs = Vec<Input>;

fn str2digit(s: &str) -> Digit {
    [
        s.contains('a'),
        s.contains('b'),
        s.contains('c'),
        s.contains('d'),
        s.contains('e'),
        s.contains('f'),
        s.contains('g'),
    ]
}

fn str2input(s: &str) -> Input {
    let (straining, stesting) = s.split_once('|').unwrap();
    let mut training: [Digit; 10] = Default::default();
    let mut testing: [Digit; 4] = Default::default();
    straining
        .split_ascii_whitespace()
        .enumerate()
        .for_each(|(i, ds)| training[i] = str2digit(ds));
    stesting
        .split_ascii_whitespace()
        .enumerate()
        .for_each(|(i, ds)| testing[i] = str2digit(ds));
    Input {
        testing: testing,
        training: training,
    }
}

fn parse(fname: &str) -> Inputs {
    std::fs::read_to_string(fname)
        .unwrap()
        .lines()
        .map(str2input)
        .collect()
}

fn part1(inputs: Inputs) -> usize {
    inputs
        .into_iter()
        .map(|inp| {
            inp.testing
                .iter()
                .map(|&digit| match digit.iter().filter(|&&x| x).count() {
                    2 | 3 | 4 | 7 => true,
                    _ => false,
                })
                .filter(|&x| x)
                .count()
        })
        .sum()
}

fn subtract_segs(d1: Digit, d2: Digit) -> Digit {
    let mut out: Digit = [false; 7];
    d1.iter()
        .zip(d2.iter())
        .enumerate()
        .for_each(|(i, (&d1s, &d2s))| out[i] = d1s && (!d2s));
    out
}

fn remap(d: Digit, dmap: [usize; 7]) -> Digit {
    let mut out: Digit = [false; 7];
    for i in 0..7 {
        out[i] = d[dmap[i]];
    }
    out
}

fn true_idx(d: Digit) -> usize {
    if d.iter().filter(|&&x| x).count() > 1 {
        panic!("should only have one true val");
    }
    d.iter()
        .enumerate()
        .find(|(_, &seg)| seg)
        .unwrap()
        .0
        .clone()
}

fn digit2char(d: Digit) -> char {
    match d {
        [true, true, true, false, true, true, true] => '0',
        [false, false, true, false, false, true, false] => '1',
        [true, false, true, true, true, false, true] => '2',
        [true, false, true, true, false, true, true] => '3',
        [false, true, true, true, false, true, false] => '4',
        [true, true, false, true, false, true, true] => '5',
        [true, true, false, true, true, true, true] => '6',
        [true, false, true, false, false, true, false] => '7',
        [true, true, true, true, true, true, true] => '8',
        [true, true, true, true, false, true, true] => '9',
        _ => '#',
    }
}

fn row_n(ds: [Digit; 10], n: usize) -> Vec<Digit> {
    let mut out = Vec::new();
    for d in ds {
        if d.iter().filter(|&&x| x).count() == n {
            out.push(d)
        }
    }
    out
}

fn col_n(ds: [Digit; 10], n: usize) -> Vec<usize> {
    (0usize..7)
        .map(|i| (i, ds.iter().filter(|&&d| d[i]).count()))
        .filter(|(_, c_i)| c_i == &n)
        .map(|(i, _)| i)
        .collect()
}

fn idx_to_digit(is: &[usize]) -> Digit {
    let mut out = [false; 7];
    for &i in is {
        out[i] = true;
    }
    out
}

fn deduce(ds: [Digit; 10]) -> [usize; 7] {
    let mut remap = [8; 7];
    let b = col_n(ds, 6)[0];
    remap[1] = b;
    let e = col_n(ds, 4)[0];
    remap[4] = e;
    let f = col_n(ds, 9)[0];
    remap[5] = f;
    let seven = row_n(ds, 3)[0];
    let one = row_n(ds, 2)[0];
    let a = true_idx(subtract_segs(seven, one));
    remap[0] = a;
    let c = true_idx(subtract_segs(one, idx_to_digit(&[f])));
    remap[2] = c;
    let four = row_n(ds, 4)[0];
    let anti_four = idx_to_digit(&[b, c, f]);
    let d = true_idx(subtract_segs(four, anti_four));
    remap[3] = d;
    let eight = row_n(ds, 7)[0];
    let anti_eight = idx_to_digit(&[a, b, c, d, e, f]);
    let g = true_idx(subtract_segs(eight, anti_eight));
    remap[6] = g;
    remap
}

fn part2(inputs: Inputs) -> usize {
    let mut total = 0;
    for input in inputs {
        let dmap = deduce(input.training);
        let num: usize = input
            .testing
            .iter()
            .map(|&d| digit2char(remap(d, dmap)))
            .collect::<String>()
            .parse()
            .unwrap();
        total += num;
    }
    total
}

#[test]
fn test_part1() {
    assert_eq!(part1(parse("test0")), 26)
}

#[test]
fn test_part2() {
    assert_eq!(part2(parse("test0")), 61229)
}

fn main() {
    println!("{:?}", part1(parse("input")));
    println!("{:?}", part2(parse("input")));
}
