use std::io;

#[derive(Debug, Clone)]
enum Rule {
    Lit(u8),
    Or(Vec<Concat>),
}

type Concat = Vec<usize>;

fn parse(lines: &[String]) -> anyhow::Result<Vec<Rule>> {
    let mut rules = vec![Rule::Lit(0); lines.len()];
    for line in lines {
        let x: Vec<&str> = line.split(':').collect();
        let rule_id = x[0].parse::<usize>()?;

        if x[1] == " \"a\"" {
            rules[rule_id] = Rule::Lit(b'a');
        } else if x[1] == " \"b\"" {
            rules[rule_id] = Rule::Lit(b'b');
        } else {
            let mut disj = vec![];
            for seq in x[1].split('|') {
                let mut cat = vec![];
                for x in seq.trim().split(' ') {
                    cat.push(x.parse::<usize>()?);
                }
                disj.push(cat);
            }
            rules[rule_id] = Rule::Or(disj);
        }
    }
    return Ok(rules);
}

fn rule_to_regex(id: usize, r: &[Rule]) -> String {
    let mut s = String::new();
    match r[id] {
        Rule::Lit(x) => s.push(x as char),
        Rule::Or(ref cats) => {
            let mut sep = "";
            for cat in cats {
                s += sep;
                s += "(";
                for x in cat {
                    s += &format!("({})", rule_to_regex(*x, r));
                }
                s += ")";
                sep = "|";
            }
        }
    }
    return s;
}

fn part1(parts: &[Vec<String>]) -> anyhow::Result<usize> {
    let mut matching: usize = 0;
    let rules = parse(&parts[0])?;
    let re = format!("^{}$", rule_to_regex(0, &rules));
    let re = regex::Regex::new(&re)?;
    for line in &parts[1] {
        if re.is_match(line) {
            matching += 1;
        }
    }
    return Ok(matching);
}

fn part2(parts: &[Vec<String>]) -> anyhow::Result<usize> {
    let mut matching: usize = 0;
    let rules = parse(&parts[0])?;
    return Ok(matching);
}

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let parts = aoc2020::read_groups_of_lines(stdin)?;

    println!("part 1: {}", part1(&parts)?);
    println!("part 2: {}", part2(&parts)?);

    return Ok(());
}
