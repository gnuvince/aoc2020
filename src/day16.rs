use std::io;
use std::ops::RangeInclusive;

fn parse_rules(rules: &[String]) -> anyhow::Result<Vec<u64>> {
    let re = regex::Regex::new(r"^[a-z ]+: (\d+)-(\d+) or (\d+)-(\d+)$")?;
    let mut allowed: Vec<u64> = Vec::new();
    for line in rules {
        if let Some(caps) = re.captures(line) {
            let a = caps.get(1).unwrap().as_str().parse::<u64>()?;
            let b = caps.get(2).unwrap().as_str().parse::<u64>()?;
            let c = caps.get(3).unwrap().as_str().parse::<u64>()?;
            let d = caps.get(4).unwrap().as_str().parse::<u64>()?;
            allowed.extend(a..=b);
            allowed.extend(c..=d);
        }
    }
    allowed.sort_unstable();
    allowed.dedup();
    return Ok(allowed);
}

fn parse_nearby_tickets(nearby: &[String]) -> anyhow::Result<Vec<Vec<u64>>> {
    let mut tickets: Vec<Vec<u64>> = Vec::new();
    for line in nearby {
        let mut this_ticket: Vec<u64> = Vec::new();
        for x in line.split(",") {
            this_ticket.push(x.parse::<u64>()?);
        }
        tickets.push(this_ticket);
    }
    return Ok(tickets);
}

fn part1(groups: &Vec<Vec<String>>) -> anyhow::Result<u64> {
    let rules = parse_rules(&groups[0])?;
    let nearby_tickets = parse_nearby_tickets(&groups[2][1..])?;
    let mut invalid: u64 = 0;

    for nt in nearby_tickets {
        for x in nt {
            if rules.binary_search(&x).is_err() {
                invalid += x;
            }
        }
    }
    return Ok(invalid);
}

fn remove_invalid(rules: &Vec<u64>, nearby: Vec<Vec<u64>>) -> Vec<Vec<u64>> {
    let mut valid: Vec<Vec<u64>> = Vec::new();
    for nt in nearby {
        if nt.iter().all(|x| rules.binary_search(x).is_ok()) {
            valid.push(nt);
        }
    }
    return valid;
}

fn parse_named_rules(
    lines: &[String],
) -> anyhow::Result<Vec<(String, RangeInclusive<u64>, RangeInclusive<u64>)>> {
    let mut rules = Vec::new();
    let re = regex::Regex::new(r"^([ a-z]+): (\d+)-(\d+) or (\d+)-(\d+)$")?;
    for line in lines {
        if let Some(caps) = re.captures(line) {
            let name = caps.get(1).unwrap().as_str().to_owned();

            let a = caps.get(2).unwrap().as_str().parse::<u64>()?;
            let b = caps.get(3).unwrap().as_str().parse::<u64>()?;
            let c = caps.get(4).unwrap().as_str().parse::<u64>()?;
            let d = caps.get(5).unwrap().as_str().parse::<u64>()?;
            rules.push((name, a..=b, c..=d));
        }
    }
    return Ok(rules);
}

fn bt(
    x: usize,
    cols: usize,
    t: &Vec<Vec<u64>>,
    r: &Vec<(String, RangeInclusive<u64>, RangeInclusive<u64>)>,
    acc: &mut Vec<usize>,
    choices: &Vec<Vec<usize>>,
) -> anyhow::Result<()> {
    if x == cols {
        return Ok(());
    }

    for j in &choices[x] {
        if !(&acc[..x]).contains(j) {
            acc[x] = *j;
            match bt(x+1, cols, t, r, acc, choices) {
                Ok(()) => return Ok(()),
                Err(_) => (),
            }
        }
    }

    anyhow::bail!("backtrack");
}

fn part2(groups: &Vec<Vec<String>>) -> anyhow::Result<u64> {
    let rules = parse_rules(&groups[0])?;
    let nearby_tickets = parse_nearby_tickets(&groups[2][1..])?;
    let valid_nearby_tickets = remove_invalid(&rules, nearby_tickets);
    let my_ticket = parse_nearby_tickets(&groups[1][1..])?;
    let named_rules = parse_named_rules(&groups[0])?;

    let rows: usize = valid_nearby_tickets.len();
    let cols: usize = valid_nearby_tickets[0].len();
    let mut config: Vec<usize> = vec![999; cols];

    let mut choices: Vec<Vec<usize>> = vec![];
    for (_, r1, r2) in named_rules.iter() {
        let mut matching_cols = vec![];
        let t = &valid_nearby_tickets;
        for col in 0 .. cols {
            if (0..rows).all(|row| r1.contains(&t[row][col]) || r2.contains(&t[row][col])) {
                matching_cols.push(col);
            }
        }
        choices.push(matching_cols);
    }

    bt(
        0,
        cols,
        &valid_nearby_tickets,
        &named_rules,
        &mut config,
        &choices,
    )?;
    let mut prod: u64 = 1;
    for c in &config[..6] {
        prod *= my_ticket[0][*c];
    }

    return Ok(prod);
}

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let groups = aoc2020::read_groups_of_lines(stdin)?;

    println!("part 1: {}", part1(&groups)?);
    println!("part 2: {}", part2(&groups)?);

    return Ok(());
}
