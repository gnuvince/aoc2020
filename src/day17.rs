use std::collections::HashSet;
use std::io;

type Point = (i64, i64, i64);
type Point4 = (i64, i64, i64, i64);

fn neighbors((x, y, z): Point) -> Vec<Point> {
    let mut ns: Vec<Point> = Vec::with_capacity(26);
    for dx in -1..=1 {
        for dy in -1..=1 {
            for dz in -1..=1 {
                if dx == 0 && dy == 0 && dz == 0 {
                    continue;
                }
                ns.push((x + dx, y + dy, z + dz));
            }
        }
    }
    return ns;
}

#[test]
fn test_neighbors() {
    assert_eq!(26, neighbors((0, 0, 0)).len());
}

fn neighbors4((x, y, z, w): Point4) -> Vec<Point4> {
    let mut ns: Vec<Point4> = Vec::with_capacity(80);
    for dx in -1..=1 {
        for dy in -1..=1 {
            for dz in -1..=1 {
                for dw in -1..=1 {
                    if dx == 0 && dy == 0 && dz == 0 && dw == 0 {
                        continue;
                    }
                    ns.push((x + dx, y + dy, z + dz, w + dw));
                }
            }
        }
    }
    return ns;
}

#[test]
fn test_neighbors4() {
    assert_eq!(80, neighbors4((0, 0, 0, 0)).len());
}

fn candidates(curr: &HashSet<Point>) -> HashSet<Point> {
    let mut to_check = curr.clone();
    for p in curr {
        for n in neighbors(*p) {
            to_check.insert(n);
        }
    }
    return to_check;
}

fn candidates4(curr: &HashSet<Point4>) -> HashSet<Point4> {
    let mut to_check = curr.clone();
    for p in curr {
        for n in neighbors4(*p) {
            to_check.insert(n);
        }
    }
    return to_check;
}

fn part1(s: &HashSet<Point>) -> usize {
    let mut curr = s.clone();
    for _ in 0..6 {
        let mut next = HashSet::new();
        let to_check = candidates(&curr);
        for p in to_check {
            let mut active_neighbors = 0;
            for n in neighbors(p) {
                active_neighbors += curr.contains(&n) as usize;
            }

            if curr.contains(&p) {
                // Currently active
                if active_neighbors == 2 || active_neighbors == 3 {
                    next.insert(p);
                }
            } else {
                // Currently inactive
                if active_neighbors == 3 {
                    next.insert(p);
                }
            }
        }
        curr = next;
    }
    return curr.len();
}

fn part2(s: &HashSet<Point4>) -> usize {
    let mut curr = s.clone();
    for _ in 0..6 {
        let mut next = HashSet::new();
        let to_check = candidates4(&curr);
        for p in to_check {
            let mut active_neighbors = 0;
            for n in neighbors4(p) {
                active_neighbors += curr.contains(&n) as usize;
            }

            if curr.contains(&p) {
                // Currently active
                if active_neighbors == 2 || active_neighbors == 3 {
                    next.insert(p);
                }
            } else {
                // Currently inactive
                if active_neighbors == 3 {
                    next.insert(p);
                }
            }
        }
        curr = next;
    }
    return curr.len();
}

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let lines = aoc2020::read_lines(stdin)?;

    let mut state: HashSet<Point> = HashSet::new();
    let mut state4: HashSet<Point4> = HashSet::new();
    for (i, line) in lines.iter().enumerate() {
        for (j, cell) in line.iter().enumerate() {
            if *cell == b'#' {
                state.insert((i as i64, j as i64, 0));
                state4.insert((i as i64, j as i64, 0, 0));
            }
        }
    }

    println!("part 1: {}", part1(&state));
    println!("part 1: {}", part2(&state4));

    return Ok(());
}
