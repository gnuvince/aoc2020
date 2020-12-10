use std::collections::HashMap;
use std::io;

fn cache_or_compute(xs: &[i32], cache: &mut HashMap<i32, u64>) -> u64 {
    if let Some(y) = cache.get(&xs[0]) {
        return *y;
    } else {
        let y = part2(xs, cache);
        cache.insert(xs[0], y);
        return y;
    }
}

fn part2(xs: &[i32], cache: &mut HashMap<i32, u64>) -> u64 {
    if xs.len() > 3 {
        if xs[3] - xs[0] <= 3 {
            let a = cache_or_compute(&xs[1..], cache);
            let b = cache_or_compute(&xs[2..], cache);
            let c = cache_or_compute(&xs[3..], cache);
            return a + b + c;
        }
        if xs[2] - xs[0] <= 3 {
            let a = cache_or_compute(&xs[1..], cache);
            let b = cache_or_compute(&xs[2..], cache);
            return a + b;
        }
        if xs[1] - xs[0] <= 3 {
            return cache_or_compute(&xs[1..], cache);
        }
        return 0;
    } else if xs.len() == 3 {
        if xs[2] - xs[0] <= 3 {
            let a = cache_or_compute(&xs[1..], cache);
            let b = cache_or_compute(&xs[2..], cache);
            return a + b;
        }
        if xs[1] - xs[0] <= 3 {
            return cache_or_compute(&xs[1..], cache);
        }
        return 0;
    } else if xs.len() == 2 {
        if xs[1] - xs[0] <= 3 {
            return cache_or_compute(&xs[1..], cache);
        }
        return 0;
    } else {
        return 1;
    }
}

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();
    let stdin = stdin.lock();

    let mut nums = aoc2020::read_numbers(stdin)?;
    nums.push(0);
    nums.sort_unstable();
    nums.push(nums[nums.len() - 1] + 3);
    let mut jolt_differences: [usize; 4] = [0; 4];
    for i in 1..nums.len() {
        let diff = nums[i] - nums[i - 1];
        jolt_differences[diff as usize] += 1;
    }

    let mut cache: HashMap<i32, u64> = HashMap::new();

    println!("part 1: {}", jolt_differences[1] * jolt_differences[3]);
    println!("part 2: {}", part2(&nums, &mut cache));

    return Ok(());
}
