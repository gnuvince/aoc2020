use std::io;

fn two_sum(target: i64, nums: &[i64]) -> Option<(usize, usize)> {
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if nums[i] + nums[j] == target {
                return Some((i, j));
            }
        }
    }

    return None;
}

fn part1(nums: &[i64]) -> Option<i64> {
    for i in 25..nums.len() {
        if two_sum(nums[i], &nums[i - 25..i]).is_none() {
            return Some(nums[i]);
        }
    }
    return None;
}

fn part2(target: i64, nums: &[i64]) -> Option<i64> {
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            let s: i64 = (&nums[i..j]).iter().sum();
            if s == target {
                let a = (&nums[i..j]).iter().min()?;
                let b = (&nums[i..j]).iter().max()?;
                return Some(a + b);
            } else if s > target {
                break;
            }
        }
    }
    return None;
}

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let nums = aoc2020::read_i64s(stdin)?;

    let x = part1(&nums);
    println!("part 1: {:?}", x);
    println!("part 2: {:?}", part2(x.unwrap(), &nums));

    return Ok(());
}
