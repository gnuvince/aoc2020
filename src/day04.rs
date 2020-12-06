use std::io::{self, BufRead};

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

fn main() -> anyhow::Result<()> {
    let hgt_re = regex::Regex::new(r"^(\d+)(cm|in)$")?;
    let hcl_re = regex::Regex::new(r"^#[0-9a-f]{6}$")?;
    let ecl_re = regex::Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$")?;
    let pid_re = regex::Regex::new(r"^[0-9]{9}$")?;

    let stdin = io::stdin();
    let stdin = stdin.lock();

    let mut p1_valid: usize = 0;
    let mut p1_curr = PassportValidation::default();
    let mut p2_valid: usize = 0;
    let mut p2_curr = PassportValidation::default();

    for line in stdin.lines() {
        let line = line?;
        if line.is_empty() {
            if p1_curr.is_valid() {
                p1_valid += 1;
            }
            p1_curr = PassportValidation::default();

            if p2_curr.is_valid() {
                p2_valid += 1;
            }
            p2_curr = PassportValidation::default();
        } else {
            for group in line.split(' ') {
                if group.starts_with("byr") {
                    p1_curr.byr = true;
                    if let Ok(yr) = (&group[4..]).parse::<i32>() {
                        p2_curr.byr = yr >= 1920 && yr <= 2002;
                    }
                }

                if group.starts_with("iyr") {
                    p1_curr.iyr = true;
                    if let Ok(yr) = (&group[4..]).parse::<i32>() {
                        p2_curr.iyr = yr >= 2010 && yr <= 2020;
                    }
                }

                if group.starts_with("eyr") {
                    p1_curr.eyr = true;
                    if let Ok(yr) = (&group[4..]).parse::<i32>() {
                        p2_curr.eyr = yr >= 2020 && yr <= 2030;
                    }
                }

                if group.starts_with("hgt") {
                    p1_curr.hgt = true;
                    if let Some(caps) = hgt_re.captures(&group[4..]) {
                        let hgt: i32 = caps.get(1).unwrap().as_str().parse::<i32>()?;
                        let units = caps.get(2).unwrap().as_str();
                        if units == "in" {
                            p2_curr.hgt = hgt >= 59 && hgt <= 76;
                        } else if units == "cm" {
                            p2_curr.hgt = hgt >= 150 && hgt <= 193;
                        }
                    }
                }

                if group.starts_with("hcl") {
                    p1_curr.hcl = true;
                    p2_curr.hcl = hcl_re.is_match(&group[4..]);
                }

                if group.starts_with("ecl") {
                    p1_curr.ecl = true;
                    p2_curr.ecl = ecl_re.is_match(&group[4..]);
                }

                if group.starts_with("pid") {
                    p1_curr.pid = true;
                    p2_curr.pid = pid_re.is_match(&group[4..]);
                }
            }
        }
    }

    if p1_curr.is_valid() {
        p1_valid += 1;
    }
    if p2_curr.is_valid() {
        p2_valid += 1;
    }

    println!("part 1: {}", p1_valid);
    println!("part 2: {}", p2_valid);

    return Ok(());
}
