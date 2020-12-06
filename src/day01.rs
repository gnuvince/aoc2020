use std::io;

fn part1(v: &[i32]) -> anyhow::Result<(i32, i32)> {
    let mut lo = 0;
    let mut hi = v.len() - 1;

    while lo < hi {
        if v[lo] + v[hi] == 2020 {
            return Ok((v[lo], v[hi]));
        } else if v[lo] + v[hi] < 2020 {
            lo += 1;
        } else {
            hi -= 1;
        }
    }
    anyhow::bail!("no pair found!");
}

// Can't quickly figure out how to do it more efficiently.
fn part2(v: &[i32]) -> anyhow::Result<(i32, i32, i32)> {
    for i in 0..v.len() {
        for j in i + 1..v.len() {
            for k in j + 1..v.len() {
                if v[i] + v[j] + v[k] == 2020 {
                    return Ok((v[i], v[j], v[k]));
                }
            }
        }
    }

    anyhow::bail!("no triple found!");
}

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let mut v: Vec<i32> = aoc2020::read_numbers(stdin)?;
    v.sort_unstable();

    let (a, b) = part1(&v)?;
    println!("part1: {} * {} = {}", a, b, a * b);

    let (x, y, z) = part2(&v)?;
    println!("part2: {} * {} * {} = {}", x, y, z, x * y * z);

    return Ok(());
}
