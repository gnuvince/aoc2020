use std::io::{self, BufRead};

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();
    let stdin = stdin.lock();

    let mut p1_sum: usize = 0;
    let mut p1_counts: [u8; 26] = [0; 26];

    let mut num_members: usize = 0;
    let mut p2_sum: usize = 0;
    let mut p2_counts: [u8; 26] = [0; 26];

    for line in stdin.lines() {
        let line = line?;

        if line.is_empty() {
            p1_sum += p1_counts.iter().sum::<u8>() as usize;
            for x in &p2_counts[..] {
                p2_sum += (*x as usize == num_members) as usize;
            }

            p1_counts = [0; 26];
            p2_counts = [0; 26];
            num_members = 0;
        } else {
            num_members += 1;
            for b in line.bytes() {
                p1_counts[(b - b'a') as usize] = 1;
                p2_counts[(b - b'a') as usize] += 1;
            }
        }
    }
    p1_sum += p1_counts.iter().sum::<u8>() as usize;
    for x in &p2_counts[..] {
        p2_sum += (*x as usize == num_members) as usize;
    }

    println!("part 1: {}", p1_sum);
    println!("part 2: {}", p2_sum);

    return Ok(());
}
