use std::io;

#[derive(Debug, Clone, Copy)]
enum InstrCode {
    Nop,
    Acc,
    Jmp,
}

#[derive(Debug, Clone, Copy)]
struct Instr {
    code: InstrCode,
    value: isize,
}

#[derive(Debug, Clone)]
struct Cpu {
    pc: isize,
    acc: isize,
    instrs: Vec<Instr>,
}

impl Cpu {
    fn step(&mut self) {
        let curr = &self.instrs[self.pc as usize];
        match curr.code {
            InstrCode::Nop => {
                self.pc += 1;
            }
            InstrCode::Acc => {
                self.acc += curr.value;
                self.pc += 1;
            }
            InstrCode::Jmp => {
                self.pc += curr.value;
            }
        }
    }
}

fn part1(cpu: &mut Cpu) -> isize {
    let mut seen: Vec<bool> = vec![false; cpu.instrs.len()];
    while !seen[cpu.pc as usize] {
        seen[cpu.pc as usize] = true;
        cpu.step();
    }
    return cpu.acc;
}

fn part2(original_cpu: &Cpu) -> anyhow::Result<isize> {
    fn run(cpu: &mut Cpu) -> (isize, bool) {
        let mut seen: Vec<bool> = vec![false; cpu.instrs.len()];
        let mut inf_loop = false;
        loop {
            if cpu.pc as usize >= cpu.instrs.len() {
                break;
            }
            if seen[cpu.pc as usize] {
                inf_loop = true;
                break;
            }
            seen[cpu.pc as usize] = true;
            cpu.step();
        }
        return (cpu.acc, inf_loop);
    }

    for i in 0..original_cpu.instrs.len() {
        match original_cpu.instrs[i].code {
            InstrCode::Nop => {
                let mut cpu = original_cpu.clone();
                cpu.instrs[i] = Instr {
                    code: InstrCode::Jmp,
                    ..cpu.instrs[i]
                };
                let (res, inf_loop) = run(&mut cpu);
                if !inf_loop {
                    return Ok(res);
                }
            }
            InstrCode::Jmp => {
                let mut cpu = original_cpu.clone();
                cpu.instrs[i] = Instr {
                    code: InstrCode::Nop,
                    ..cpu.instrs[i]
                };
                let (res, inf_loop) = run(&mut cpu);
                if !inf_loop {
                    return Ok(res);
                }
            }
            _ => {}
        }
    }
    anyhow::bail!("cannot fix the infinite loop");
}

fn parse(lines: Vec<String>) -> anyhow::Result<Vec<Instr>> {
    let mut instrs = Vec::with_capacity(lines.len());
    for line in lines {
        let parts: Vec<&str> = line.split(' ').collect();
        anyhow::ensure!(
            parts.len() == 2,
            "instruction {} has more than two parts",
            line
        );
        let code = match parts[0] {
            "nop" => InstrCode::Nop,
            "acc" => InstrCode::Acc,
            "jmp" => InstrCode::Jmp,
            x => anyhow::bail!("{} is not a valid instruction code", x),
        };
        let value = parts[1].parse::<isize>()?;
        instrs.push(Instr { code, value });
    }
    return Ok(instrs);
}

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let lines = aoc2020::read_lines_string(stdin)?;
    let instrs = parse(lines)?;
    let mut cpu1 = Cpu {
        pc: 0,
        acc: 0,
        instrs: instrs.clone(),
    };

    let cpu2 = Cpu {
        pc: 0,
        acc: 0,
        instrs,
    };

    println!("part 1: {}", part1(&mut cpu1));
    println!("part 2: {}", part2(&cpu2)?);

    return Ok(());
}
