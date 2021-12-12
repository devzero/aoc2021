#[derive(Debug)]
enum ParseError {
    Ok,
    Incomplete(Vec<char>),
    Corrupted(char, char),
}
type ParseErrors = Vec<ParseError>;

fn opposite(c: char) -> char {
    match c {
        '}' => '{',
        '{' => '}',
        ']' => '[',
        '[' => ']',
        '(' => ')',
        ')' => '(',
        '<' => '>',
        '>' => '<',
        _ => panic!(),
    }
}

fn parse_line(l: &str) -> ParseError {
    let mut state: Vec<char> = Vec::new();
    for c in l.chars() {
        match c {
            '{' | '(' | '<' | '[' => state.push(c),
            '}' | ')' | '>' | ']' => {
                if state[state.len() - 1] != opposite(c) {
                    return ParseError::Corrupted(opposite(state[state.len() - 1]), c);
                } else {
                    state.pop();
                }
            }
            _ => panic!(),
        }
    }
    if state.len() > 0 {
        state.reverse();
        state = state.into_iter().map(opposite).collect();
        ParseError::Incomplete(state.clone())
    } else {
        ParseError::Ok
    }
}

fn parse(fname: &str) -> ParseErrors {
    std::fs::read_to_string(fname)
        .unwrap()
        .lines()
        .map(|l| parse_line(l))
        .collect()
}

fn part1(p: ParseErrors) -> u128 {
    p.into_iter()
        .map(|pe| match pe {
            ParseError::Corrupted(_got, expected) => match expected {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => 0,
            },
            ParseError::Ok => 0,
            ParseError::Incomplete(_) => 0,
        })
        .sum()
}

fn points(c: char) -> u128 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!(),
    }
}

fn part2(p: ParseErrors) -> u128 {
    let mut scores: Vec<u128> = p
        .into_iter()
        .filter(|pe| {
            if let ParseError::Incomplete(_) = pe {
                true
            } else {
                false
            }
        })
        .map(|pe| {
            if let ParseError::Incomplete(s) = pe {
                s.into_iter().fold(0, |acc, c| 5 * acc + points(c))
            } else {
                0
            }
        })
        .collect();
    scores.sort();
    let mid = &scores.len() / 2;
    scores[mid]
}

#[test]
fn test_part1() {
    assert_eq!(part1(parse("test0")), 26397)
}

#[test]
fn test_part2() {
    assert_eq!(part2(parse("test0")), 288957)
}

fn main() {
    println!("{:?}", part1(parse("input")));
    println!("{:?}", part2(parse("input")));
}
