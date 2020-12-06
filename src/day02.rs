use std::io::{self, BufRead};

struct PasswordRecord {
    lo: usize,
    hi: usize,
    ch: u8,
    password: Vec<u8>,
}

fn part1(v: &[PasswordRecord]) -> usize {
    let mut matching: usize = 0;
    for r in v {
        let mut count: usize = 0;
        for c in &r.password {
            if *c == r.ch {
                count += 1;
            }
        }
        if count >= r.lo && count <= r.hi {
            matching += 1;
        }
    }
    return matching;
}

fn part2(v: &[PasswordRecord]) -> usize {
    let mut matching: usize = 0;
    for r in v {
        let m1 = r.password[r.lo - 1] == r.ch;
        let m2 = r.password[r.hi - 1] == r.ch;
        matching += (m1 as usize) ^ (m2 as usize);
    }
    return matching;
}

fn main() -> anyhow::Result<()> {
    const PATTERN: &str = r"(\d+)-(\d+) ([a-z]): ([a-z]+)";
    let re = regex::Regex::new(PATTERN)?;

    let stdin = io::stdin();
    let stdin = stdin.lock();

    let mut password_records: Vec<PasswordRecord> = Vec::new();

    for line in stdin.lines() {
        let line = line?;
        let caps = re.captures(&line).unwrap();

        let lo = caps.get(1).unwrap().as_str().parse::<usize>()?;
        let hi = caps.get(2).unwrap().as_str().parse::<usize>()?;
        let ch = caps.get(3).unwrap().as_str().as_bytes()[0];
        let password = caps.get(4).unwrap().as_str().as_bytes().to_owned();

        password_records.push(PasswordRecord {
            lo,
            hi,
            ch,
            password,
        });
    }

    println!("part 1: {}", part1(&password_records));
    println!("part 2: {}", part2(&password_records));

    return Ok(());
}
