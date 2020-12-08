use std::collections::{HashMap, HashSet};
use std::io;

type Graph = HashMap<String, Vec<(usize, String)>>;

fn dfs(g: &Graph, bag: &str) -> bool {
    if let Some(v) = g.get(bag) {
        for (_, subbag) in v {
            if subbag == "shiny gold" {
                return true;
            }
            if dfs(g, subbag) {
                return true;
            }
        }
    }
    return false;
}

fn part1(g: &Graph) -> usize {
    let mut bags: HashSet<&str> = HashSet::new();
    for (bag, _) in g {
        if dfs(g, bag) {
            bags.insert(bag);
        }
    }
    return bags.len();
}

fn dfs2(g: &Graph, bag: &str, mult: usize) -> usize {
    if let Some(v) = g.get(bag) {
        let mut count: usize = 0;
        for (n, subbag) in v {
            count += n + dfs2(g, subbag, *n);
        }
        return mult * count;
    }
    return 0;
}

fn part2(g: &Graph) -> usize {
    return dfs2(g, "shiny gold", 1);
}

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();
    let stdin = stdin.lock();

    let mut graph: Graph = HashMap::new();
    let lines = aoc2020::read_lines_string(stdin)?;

    for line in lines {
        let words: Vec<String> = line.split(' ').map(|x| x.to_owned()).collect();
        let subject: String = format!("{} {}", words[0], words[1]);
        if line.contains("no other bags") {
            graph.insert(subject, vec![]);
        } else {
            let num_subjects = (words.len() - 4) / 4;
            for i in 0..num_subjects {
                let j = 4 + 4 * i;
                let count: usize = words[j].parse::<usize>()?;
                let object = format!("{} {}", words[j + 1], words[j + 2]);
                let links = graph.entry(subject.clone()).or_insert(vec![]);
                links.push((count, object));
            }
        }
    }

    println!("part 1: {}", part1(&graph));
    println!("part 2: {}", part2(&graph));

    return Ok(());
}
