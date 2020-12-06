use std::io;

fn slide_down(v: &Vec<Vec<u8>>, right: usize, down: usize) -> usize {
    let width: usize = v[0].len();
    let mut trees: usize = 0;
    let mut i: usize = 0;
    let mut j: usize = 0;

    while i < v.len() {
        trees += (v[i][j] == b'#') as usize;
        i += down;
        j = (j + right) % width;
    }

    return trees;
}

fn part1(v: &Vec<Vec<u8>>) -> usize {
    return slide_down(v, 3, 1);
}

fn part2(v: &Vec<Vec<u8>>) -> usize {
    return slide_down(v, 1, 1)
        * slide_down(v, 3, 1)
        * slide_down(v, 5, 1)
        * slide_down(v, 7, 1)
        * slide_down(v, 1, 2);
}

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();
    let stdin = stdin.lock();

    let lines = aoc2020::read_lines(stdin)?;

    println!("part 1: {}", part1(&lines));
    println!("part 2: {}", part2(&lines));

    return Ok(());
}
