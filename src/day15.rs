use std::io;

fn parts(turns: usize, nums: &Vec<i32>) -> usize {
    let mut cache: Vec<i32> = vec![-1; turns];
    for i in 0..nums.len() - 1 {
        cache[nums[i] as usize] = (i+1) as i32;
    }

    let mut curr: usize = nums[nums.len() - 1] as usize;

    for t in nums.len()..turns {
        match cache[curr] {
            -1 => {
                cache[curr] = t as i32;
                curr = 0;
            }
            x => {
                cache[curr] = t as i32;
                curr = (t as i32 - x) as usize;
            }
        }
    }
    return curr;
}

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let lines = aoc2020::read_lines_string(stdin)?;
    let mut nums: Vec<i32> = vec![];
    for s in lines[0].split(',') {
        nums.push(s.parse::<i32>()?);
    }

    println!("part 1: {}", parts(2020, &nums));
    println!("part 2: {}", parts(30000000, &nums));

    return Ok(());
}
