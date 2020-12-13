use std::io;

fn part1(lines: &[String]) -> anyhow::Result<i64> {
    let time: i64 = lines[0].parse::<i64>()?;
    let mut wait_time: i64 = i64::max_value();
    let mut busid: i64 = -1;
    for this_busid in lines[1].split(',') {
        if this_busid == "x" {
            continue;
        }
        let this_busid: i64 = this_busid.parse::<i64>()?;
        let this_wait_time = this_busid - (time % this_busid);
        if this_wait_time < wait_time {
            wait_time = this_wait_time;
            busid = this_busid;
        }
    }
    return Ok(wait_time * busid);
}

fn part2(lines: &[String]) -> anyhow::Result<i64> {
    let mut busids: Vec<(i64, i64)> = vec![];
    for (i, id) in lines[1].split(',').enumerate() {
        if id == "x" {
            continue;
        }
        busids.push((i as i64, id.parse::<i64>()?));
    }

    let mut t: i64 = 0;
    let mut iters: i64 = 0; // just for debugging

    loop {
        iters += 1;
        let mut jump: i64 = 1;
        let mut okay: usize = 0;
        for (m, b) in &busids {
            if (t + m) % b == 0 {
                jump *= b;
            } else {
                break;
            }
            okay += 1;
        }

        if okay == busids.len() {
            break;
        } else {
            t += jump;
        }
    }

    dbg!(iters);
    return Ok(t);
}

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let lines = aoc2020::read_lines_string(stdin)?;

    println!("part 1: {}", part1(&lines)?);
    println!("part 2: {}", part2(&lines)?);

    return Ok(());
}
