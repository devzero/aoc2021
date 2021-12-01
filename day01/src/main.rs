use std::fs;
use itertools::zip;

type Data = Vec<u32>;

fn parse(fname: &str) -> Data {
    let input :String = String::from_utf8(fs::read(fname).expect("missing file")).unwrap();
    input.lines().map(|x| { x.parse().unwrap()}).collect()
}

#[test]
fn test_star1(){
    assert_eq!(star1(parse("test0")), 7)
}

fn star1(data: Data) -> usize {
    data.into_iter().fold(
        (u32::MAX,0), 
        | (prev, val), next | -> (u32, usize) {
            if next>prev {
                (next, val+1)
            } else { 
                (next, val)}
            } 
        ).1
}

#[test]
fn test_star2(){
    assert_eq!(star2(parse("test0")), 5)
}

fn star2(data: Data) -> usize {
    let windowed:Vec<u32> = zip(zip(&data, &data[1..]), &data[2..]).map(|((x, y), z)| x+y+z).collect();
    star1(windowed)
}

fn main() {
    println!("{:?}", star1(parse("input")));
    println!("{:?}", star2(parse("input")));
}
