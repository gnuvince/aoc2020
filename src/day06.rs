use std::io;

fn part1(g: &Vec<Vec<String>>) -> usize {
    let mut sum: usize = 0;
    let mut counts: [u8; 26] = [0; 26];

    for group in g {
        for line in group {
            for b in line.bytes() {
                counts[(b - b'a') as usize] = 1;
            }
        }
        sum += counts.iter().sum::<u8>() as usize;
        counts = [0; 26];
    }

    return sum;
}

fn part2(g: &Vec<Vec<String>>) -> usize {
    let mut sum: usize = 0;
    let mut counts: [u8; 26] = [0; 26];

    for group in g {
        for line in group {
            for b in line.bytes() {
                counts[(b - b'a') as usize] += 1;
            }
        }
        for x in &counts[..] {
            sum += (*x as usize == group.len()) as usize;
        }
        counts = [0; 26];
    }

    return sum;
}

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();
    let stdin = stdin.lock();

    let groups = aoc2020::read_groups_of_lines(stdin)?;

    println!("part 1: {}", part1(&groups));
    println!("part 2: {}", part2(&groups));

    return Ok(());
}
