use std::collections::HashMap;
use std::io;

#[derive(Debug, Clone)]
enum Instr {
    Mask(String),
    Mem(u64, u64),
}

fn parse(lines: Vec<String>) -> anyhow::Result<Vec<Instr>> {
    let mask_re = regex::Regex::new(r"^mask = ([01X]{36})$")?;
    let mem_re = regex::Regex::new(r"^mem\[(\d+)\] = (\d+)$")?;
    let mut instrs: Vec<Instr> = Vec::new();
    for line in lines {
        if let Some(caps) = mask_re.captures(&line) {
            let mask_str = caps.get(1).unwrap().as_str().to_owned();
            instrs.push(Instr::Mask(mask_str));
        } else if let Some(caps) = mem_re.captures(&line) {
            let addr = caps.get(1).unwrap().as_str().parse::<u64>()?;
            let value = caps.get(2).unwrap().as_str().parse::<u64>()?;
            instrs.push(Instr::Mem(addr, value));
        }
    }

    return Ok(instrs);
}

fn part1(instrs: &[Instr]) -> u64 {
    let mut mask = String::new();
    let mut mem: HashMap<u64, u64> = HashMap::new();
    for instr in instrs {
        match *instr {
            Instr::Mask(ref m) => mask = m.clone(),
            Instr::Mem(addr, v) => {
                let mut u = v;
                for (i, b) in mask.as_bytes().iter().enumerate() {
                    if *b == b'0' {
                        u &= !(1 << (35 - i));
                    } else if *b == b'1' {
                        u |= 1 << (35 - i);
                    }
                }
                mem.insert(addr, u);
            }
        }
    }
    let mut s = 0;
    for (_, v) in mem {
        s += v;
    }
    return s;
}

fn addrs(addr: u64, mask: &[u8]) -> Vec<u64> {
    let mut out = vec![addr];
    for (i, b) in mask.iter().enumerate() {
        if *b == b'1' {
            for a in &mut out {
                *a |= 1 << (35 - i);
            }
        } else if *b == b'X' {
            let mut cp = out.clone();
            for a in &mut out {
                *a |= 1 << (35 - i);
            }
            for a in &mut cp {
                *a &= !(1 << (35 - i));
            }
            out.extend_from_slice(&cp);
        }
    }
    return out;
}

fn part2(instrs: &[Instr]) -> u64 {
    let mut mask = String::new();
    let mut mem = HashMap::new();
    for instr in instrs {
        match *instr {
            Instr::Mask(ref m) => mask = m.clone(),
            Instr::Mem(addr, v) => {
                let all_addrs = addrs(addr, mask.as_bytes());
                for a in all_addrs {
                    mem.insert(a, v);
                }
            }
        }
    }
    let mut s = 0;
    for (_, v) in mem {
        s += v;
    }
    return s;
}

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let instrs = parse(aoc2020::read_lines_string(stdin)?)?;

    println!("part 1: {}", part1(&instrs));
    println!("part 2: {}", part2(&instrs));

    return Ok(());
}
