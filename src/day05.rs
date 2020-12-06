use std::io;

// Basically binary. B and R = 1, F and L = 0.
fn seat_id(line: &[u8]) -> usize {
    let mut x: usize = 0;
    for d in line {
        x = (x << 1) | ((*d == b'B' || *d == b'R') as usize);
    }
    return x;
}

fn part1(v: &Vec<Vec<u8>>) -> Option<usize> {
    v.iter().map(|l| seat_id(l)).max()
}

fn part2(v: &Vec<Vec<u8>>) -> Option<usize> {
    let mut seats: Vec<usize> = v.iter().map(|l| seat_id(l)).collect();
    seats.sort_unstable();
    for i in 1..seats.len() {
        if seats[i] - seats[i - 1] == 2 {
            return Some(seats[i] - 1);
        }
    }
    return None;
}

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();
    let stdin = stdin.lock();

    let lines = aoc2020::read_lines(stdin)?;

    println!("part 1: {:?}", part1(&lines));
    println!("part 2: {:?}", part2(&lines));

    return Ok(());
}

#[test]
fn test_seat_ids() {
    assert_eq!(357, seat_id(b"FBFBBFFRLR"));
    assert_eq!(567, seat_id(b"BFFFBBFRRR"));
    assert_eq!(119, seat_id(b"FFFBBBFRRR"));
    assert_eq!(820, seat_id(b"BBFFBBFRLL"));
}
