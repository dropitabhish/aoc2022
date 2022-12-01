// https://adventofcode.com/2022/day/1

use std::fs::read_to_string;
fn main() {
    let input = read_to_string("src/input.txt").expect("File not found");
    let inp_vec = input.trim().split("\n\n").collect::<Vec<&str>>();

    let mut store: Vec<u32> = Vec::new();

    // Part 1
    for i in &inp_vec {
        let temp: u32 = i
            .trim()
            .split('\n')
            .map(|x| x.parse::<u32>().unwrap())
            .reduce(|acc, x| acc + x)
            .unwrap();
        store.push(temp);
    }
    store.sort_by(|a, b| b.cmp(a));
    let max: u32 = *(store.first().unwrap());
    println!("{:?}", max);

    // Part 2
    let sum_top_three: u32 = store.iter().take(3).sum();
    println!("{:?}", sum_top_three);
}
