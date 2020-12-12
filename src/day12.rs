use std::io;

#[derive(Debug)]
struct Instr {
    code: u8,
    value: i64,
}

fn parse(lines: Vec<String>) -> anyhow::Result<Vec<Instr>> {
    let mut instrs: Vec<Instr> = vec![];
    for line in lines {
        let code: u8 = line.as_bytes()[0];
        let value: i64 = (&line[1..]).parse::<i64>()?;
        instrs.push(Instr { code, value });
    }
    return Ok(instrs);
}

fn part1(instrs: &[Instr]) -> i64 {
    let mut pos = (0_i64, 0_i64);
    let mut bearing = (1_i64, 0_i64);

    for i in instrs {
        match i {
            Instr { code: b'F', value } => {
                pos.0 += value * bearing.0;
                pos.1 += value * bearing.1;
            }
            Instr { code: b'N', value } => pos.1 += value,
            Instr { code: b'S', value } => pos.1 -= value,
            Instr { code: b'E', value } => pos.0 += value,
            Instr { code: b'W', value } => pos.0 -= value,
            Instr { code: b'L', value } => {
                let ninety_turns = value / 90;
                for _ in 0..ninety_turns {
                    bearing = (-bearing.1, bearing.0);
                }
            }
            Instr { code: b'R', value } => {
                let ninety_turns = value / 90;
                for _ in 0..ninety_turns {
                    bearing = (bearing.1, -bearing.0);
                }
            }
            _ => panic!("instr: {:?}", i),
        }
    }

    return pos.0.abs() + pos.1.abs();
}

fn part2(instrs: &[Instr]) -> i64 {
    let mut pos = (0_i64, 0_i64);
    let mut wp = (10_i64, 1_i64);

    for i in instrs {
        match i {
            Instr { code: b'F', value } => {
                pos.0 += value * wp.0;
                pos.1 += value * wp.1;
            }
            Instr { code: b'N', value } => wp.1 += value,
            Instr { code: b'S', value } => wp.1 -= value,
            Instr { code: b'E', value } => wp.0 += value,
            Instr { code: b'W', value } => wp.0 -= value,
            Instr { code: b'L', value } => {
                let ninety_turns = value / 90;
                for _ in 0..ninety_turns {
                    wp = (-wp.1, wp.0);
                }
            }
            Instr { code: b'R', value } => {
                let ninety_turns = value / 90;
                for _ in 0..ninety_turns {
                    wp = (wp.1, -wp.0);
                }
            }
            _ => panic!("wtf {:?}", i),
        }
    }

    return pos.0.abs() + pos.1.abs();
}

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let lines = aoc2020::read_lines_string(stdin)?;
    let instrs = parse(lines)?;

    println!("part 1: {}", part1(&instrs));
    println!("part 2: {}", part2(&instrs));

    return Ok(());
}
