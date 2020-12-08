use std::io;

#[derive(Debug, Clone, Copy)]
enum InstrCode {
    Nop,
    Acc,
    Jmp,
}

impl InstrCode {
    fn flip(&self) -> Self {
        use InstrCode::*;
        match *self {
            Nop => Jmp,
            Jmp => Nop,
            Acc => Acc,
        }
    }
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

    fn run(&mut self) -> (isize, bool) {
        let mut seen: Vec<bool> = vec![false; self.instrs.len()];
        while (self.pc as usize) < self.instrs.len() && !seen[self.pc as usize] {
            seen[self.pc as usize] = true;
            self.step();
        }
        return (self.acc, (self.pc as usize) < self.instrs.len());
    }
}

fn part1(cpu: &mut Cpu) -> isize {
    return cpu.run().0;
}

fn part2(original_cpu: &Cpu) -> anyhow::Result<isize> {
    for i in 0..original_cpu.instrs.len() {
        match original_cpu.instrs[i].code {
            InstrCode::Nop | InstrCode::Jmp => {
                let mut cpu = original_cpu.clone();
                cpu.instrs[i].code = cpu.instrs[i].code.flip();
                let (res, inf_loop) = cpu.run();
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
