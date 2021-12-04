use std::fs;
use array2d::Array2D;

fn parse(fname: &str) -> Vec<Vec<bool>> {
    String::from_utf8(fs::read(fname).expect("missing file"))
        .unwrap()
        .lines()
        .map(|x| 
            x
                .chars()
                .map(|c| 
                    match c {
                        '0' => false,
                        '1' => true,
                        _ => panic!("bad char")
                    }
                )
                .collect::<Vec<bool>>()
        )
        .collect()
}

#[test]
fn test_part1(){
    assert_eq!(part1(parse("test0")), 230)
}

fn part1(bits: Vec<Vec<bool>>) -> usize {
    let half_of_entries = bits.len()/2;
    let array = Array2D::from_rows(&bits);
    let gamma = array
        .as_columns()
        .iter()
        .map(|col| (
            col
                .iter()
                .filter(|&&x| x)
                .count()
            )>half_of_entries
        )
        .fold(0usize, |acc, val| 
            if val {acc*2+1} else {acc*2}
        );
    let epsilon = (!gamma)%(1<<array.num_columns());
    gamma*epsilon
}

#[test]
fn test_part2(){
    assert_eq!(part2(parse("test0")), 230)
}

fn filter_round(bits: Vec<Vec<bool>>, col: usize, co2: bool) -> Vec<Vec<bool>> { 
    let arr = Array2D::from_rows(&bits);
    let ones = arr.column_iter(col).filter(|&&x| x==true).count();
    let zeroes = bits.len()-ones;
    let least_common_value = zeroes>ones;
    let most_common_value = ones>=zeroes;
    let myfilter_fn;
    myfilter_fn = |v: &&bool| if co2 {
        v==&&least_common_value
    } else {
        v==&&most_common_value
    };
    bits.iter().zip(arr.column_iter(col))
        .filter(|(_, val)| myfilter_fn(val))
        .map(|(row, _)| row.clone())
        .collect()
}

fn elem_generator(bits: &Vec<Vec<bool>>, co2: bool) -> usize {
    let mut results = bits.clone();
    let mut col = 0;
    while results.len()>1{
        results = filter_round(results, col, co2);
        col += 1;
    } 
    results[0]
        .iter()
        .fold(0usize, |acc, &val| 
            if val {acc*2+1} else {acc*2}
        )
}

fn part2(bits: Vec<Vec<bool>>) -> usize {
    let oxy = elem_generator(&bits, false);
    let co2 = elem_generator(&bits, true);
    oxy*co2
}

fn main() {
    println!("{:?}", part1(parse("input")));
    println!("{:?}", part2(parse("input")));
}
