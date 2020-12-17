use std::collections::HashMap;
use std::io;

fn parts(turns: usize, nums: &Vec<usize>) -> usize {
    let mut cache: HashMap<usize, usize> = HashMap::new();
    for i in 0..nums.len() - 1 {
        cache.insert(nums[i], i + 1);
    }
    let mut prev: usize = nums[nums.len() - 1];

    for t in nums.len()..turns {
        if let Some(x) = cache.get(&prev) {
            let p = prev;
            prev = t - x;
            cache.insert(p, t);
        } else {
            cache.insert(prev, t);
            prev = 0;
        }
    }

    return prev;
}

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let lines = aoc2020::read_lines_string(stdin)?;
    let mut nums: Vec<usize> = vec![];
    for s in lines[0].split(',') {
        nums.push(s.parse::<usize>()?);
    }

    println!("part 1: {}", parts(2020, &nums));
    println!("part 2: {}", parts(30000000, &nums));

    return Ok(());
}
