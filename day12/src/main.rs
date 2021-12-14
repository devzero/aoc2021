use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
struct Graph {
    names: HashMap<String, usize>,
    matrix: HashMap<usize, Vec<usize>>,
    big: HashSet<usize>,
    start: usize,
    end: usize,
}

fn parse(fname: &str) -> Graph {
    let mut cur_num = 0;
    let mut g = Graph {
        names: HashMap::new(),
        matrix: HashMap::new(),
        big: HashSet::new(),
        start: 0,
        end: 0,
    };

    let strnodes = std::fs::read_to_string(fname)
        .unwrap()
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .map(|(s1, s2)| (s1.to_owned(), s2.to_owned()))
        .collect::<Vec<(String, String)>>();

    for (lname, rname) in &strnodes {
        for name in [lname, rname] {
            if !g.names.contains_key(name) {
                g.names.insert(name.to_string(), cur_num);
                g.matrix.insert(cur_num, Vec::new());
                if name.chars().all(|c| c.is_uppercase()) {
                    g.big.insert(cur_num);
                }
                if name == "start" {
                    g.start = cur_num;
                }
                if name == "end" {
                    g.end = cur_num;
                }
                cur_num += 1;
            }
        }
        g.matrix
            .get_mut(&g.names[lname])
            .unwrap()
            .push(g.names[rname]);
        g.matrix
            .get_mut(&g.names[rname])
            .unwrap()
            .push(g.names[lname]);
    }
    g
}

fn count_paths(g: &Graph, start: usize, visits: &mut HashMap<usize, u32>) -> u128 {
    visits.insert(start, visits.get(&start).unwrap_or(&0) + 1);
    if start == g.end {
        return 1;
    } else {
        g.matrix
            .get(&start)
            .unwrap()
            .clone()
            .into_iter()
            .filter(|w| g.big.contains(w) || visits.get(w).unwrap_or(&0) < &1)
            .map(|w| count_paths(g, w, &mut visits.clone()))
            .sum()
    }
}
fn part1(g: &Graph) -> usize {
    count_paths(g, g.start, &mut HashMap::new()) as usize
}

fn count_paths2(
    g: &Graph,
    start: usize,
    path: &mut Vec<usize>,
    visits: &mut HashMap<usize, u32>,
) -> u128 {
    visits.insert(start, visits.get(&start).unwrap_or(&0) + 1);
    path.push(start);
    if start == g.end {
        return 1;
    } else {
        g.matrix
            .get(&start)
            .unwrap()
            .clone()
            .into_iter()
            .filter(|w| {
                let wvisits = visits.get(w).unwrap_or(&0);
                let mut counts: HashMap<usize, u32> = HashMap::new();
                for entity in path.clone() {
                    *counts.entry(entity).or_insert(0) += 1;
                }
                *counts.entry(*w).or_insert(0) += 1;
                if counts
                    .into_iter()
                    .filter(|(node, __)| {
                        node != &g.start && node != &g.end && !g.big.contains(node)
                    })
                    .filter(|(_, count)| count > &1u32)
                    .count()
                    > 1
                {
                    return false;
                }
                g.big.contains(w)
                    || wvisits < &1
                    || ((w != &g.start) && (w != &g.end) && (wvisits < &2))
            })
            .map(|w| count_paths2(g, w, &mut path.clone(), &mut visits.clone()))
            .sum()
    }
}
fn part2(g: &Graph) -> usize {
    count_paths2(g, g.start, &mut vec![], &mut HashMap::new()) as usize
}
#[test]
fn test0_part1() {
    assert_eq!(part1(&parse("test0")), 10)
}
#[test]
fn test1_part1() {
    assert_eq!(part1(&parse("test1")), 19)
}
#[test]
fn test2_part1() {
    assert_eq!(part1(&parse("test2")), 226)
}
#[test]
fn test0_part2() {
    assert_eq!(part2(&parse("test0")), 36)
}
#[test]
fn test1_part2() {
    assert_eq!(part2(&parse("test1")), 103)
}
#[test]
fn test2_part2() {
    assert_eq!(part2(&parse("test2")), 3509)
}
fn main() {
    println!("{:?}", part1(&parse("input")));
    println!("{:?}", part2(&parse("input")));
}
