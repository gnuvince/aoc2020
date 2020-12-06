use std::io;

#[derive(Default)]
struct PassportValidation {
    byr: bool,
    iyr: bool,
    eyr: bool,
    hgt: bool,
    hcl: bool,
    ecl: bool,
    pid: bool,
}

impl PassportValidation {
    fn is_valid(&self) -> bool {
        self.byr && self.iyr && self.eyr && self.hgt && self.hcl && self.ecl && self.pid
    }
}

fn part1(g: &Vec<Vec<String>>) -> usize {
    let mut valid: usize = 0;
    for group in g {
        let mut pv = PassportValidation::default();
        for line in group {
            pv.byr = pv.byr || line.contains("byr");
            pv.iyr = pv.iyr || line.contains("iyr");
            pv.eyr = pv.eyr || line.contains("eyr");
            pv.hgt = pv.hgt || line.contains("hgt");
            pv.hcl = pv.hcl || line.contains("hcl");
            pv.ecl = pv.ecl || line.contains("ecl");
            pv.pid = pv.pid || line.contains("pid");
        }
        valid += pv.is_valid() as usize;
    }
    return valid;
}

fn part2(g: &Vec<Vec<String>>) -> anyhow::Result<usize> {
    let hgt_re = regex::Regex::new(r"^(\d+)(cm|in)$")?;
    let hcl_re = regex::Regex::new(r"^#[0-9a-f]{6}$")?;
    let ecl_re = regex::Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$")?;
    let pid_re = regex::Regex::new(r"^[0-9]{9}$")?;

    let mut valid: usize = 0;
    for group in g {
        let mut pv = PassportValidation::default();
        for line in group {
            for chunk in line.split(' ') {
                if chunk.starts_with("byr") {
                    if let Ok(yr) = (&chunk[4..]).parse::<i32>() {
                        pv.byr = yr >= 1920 && yr <= 2002;
                    }
                }

                if chunk.starts_with("iyr") {
                    if let Ok(yr) = (&chunk[4..]).parse::<i32>() {
                        pv.iyr = yr >= 2010 && yr <= 2020;
                    }
                }

                if chunk.starts_with("eyr") {
                    if let Ok(yr) = (&chunk[4..]).parse::<i32>() {
                        pv.eyr = yr >= 2020 && yr <= 2030;
                    }
                }

                if chunk.starts_with("hgt") {
                    if let Some(caps) = hgt_re.captures(&chunk[4..]) {
                        let hgt: i32 = caps.get(1).unwrap().as_str().parse::<i32>()?;
                        let units = caps.get(2).unwrap().as_str();
                        if units == "in" {
                            pv.hgt = hgt >= 59 && hgt <= 76;
                        } else if units == "cm" {
                            pv.hgt = hgt >= 150 && hgt <= 193;
                        }
                    }
                }

                if chunk.starts_with("hcl") {
                    pv.hcl = hcl_re.is_match(&chunk[4..]);
                }

                if chunk.starts_with("ecl") {
                    pv.ecl = ecl_re.is_match(&chunk[4..]);
                }

                if chunk.starts_with("pid") {
                    pv.pid = pid_re.is_match(&chunk[4..]);
                }
            }
        }
        valid += pv.is_valid() as usize;
    }
    return Ok(valid);
}

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let groups = aoc2020::read_groups_of_lines(stdin)?;
    println!("part 1: {}", part1(&groups));
    println!("part 2: {}", part2(&groups)?);
    return Ok(());
}
