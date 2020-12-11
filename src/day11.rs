use std::io;

fn neighbors(i: isize, j: isize, rows: isize, cols: isize) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    let ds: Vec<(isize, isize)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    for (di, dj) in ds {
        let i2 = i + di;
        let j2 = j + dj;
        if i2 < 0 || i2 >= rows {
            continue;
        }
        if j2 < 0 || j2 >= cols {
            continue;
        }
        out.push((i2 as usize, j2 as usize));
    }
    out.sort_unstable();
    out.dedup();
    return out;
}

fn part1(cells: Vec<Vec<u8>>) -> u64 {
    let mut cells = cells.clone();
    let mut changed = true;
    let rows = cells.len();
    let cols = cells[0].len();
    let mut ns: Vec<Vec<Vec<(usize, usize)>>> = vec![vec![vec![]; cols]; rows];

    // Pre-compute all neighbors
    for i in 0..rows {
        for j in 0..cols {
            ns[i][j] = neighbors(i as isize, j as isize, rows as isize, cols as isize);
        }
    }

    while changed {
        let mut new_cells = cells.clone();
        for i in 0..rows {
            for j in 0..cols {
                if cells[i][j] == b'.' {
                    continue;
                }
                let mut occupied = 0;
                for (i2, j2) in &ns[i][j] {
                    occupied += (cells[*i2][*j2] == b'#') as usize;
                }
                if occupied >= 4 {
                    new_cells[i][j] = b'L';
                } else if occupied == 0 {
                    new_cells[i][j] = b'#';
                }
            }
        }
        changed = cells != new_cells;
        cells = new_cells;
    }
    let mut occ: u64 = 0;
    for line in cells {
        for cell in line {
            occ += (cell == b'#') as u64;
        }
    }
    return occ;
}

fn is_occupied(
    cells: &Vec<Vec<u8>>,
    mut i: isize,
    mut j: isize,
    di: isize,
    dj: isize,
    rows: isize,
    cols: isize,
) -> bool {
    loop {
        i += di;
        j += dj;

        if i < 0 || i >= rows {
            break;
        }
        if j < 0 || j >= cols {
            break;
        }

        match cells[i as usize][j as usize] {
            b'.' => {}
            b'#' => return true,
            b'L' => return false,
            _ => panic!("wtf"),
        }
    }

    return false;
}

fn part2(cells: Vec<Vec<u8>>) -> u64 {
    let mut cells = cells.clone();
    let mut changed = true;
    let rows = cells.len();
    let cols = cells[0].len();

    let dirs: Vec<(isize, isize)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    while changed {
        let mut new_cells = cells.clone();
        for i in 0..rows {
            for j in 0..cols {
                if cells[i][j] == b'.' {
                    continue;
                }
                let mut occupied = 0;
                for (di, dj) in &dirs {
                    occupied += (is_occupied(
                        &cells,
                        i as isize,
                        j as isize,
                        *di,
                        *dj,
                        rows as isize,
                        cols as isize,
                    )) as u64;
                }
                if occupied >= 5 {
                    new_cells[i][j] = b'L';
                } else if occupied == 0 {
                    new_cells[i][j] = b'#';
                }
            }
        }
        changed = cells != new_cells;
        cells = new_cells;
    }
    let mut occ: u64 = 0;
    for line in cells {
        for cell in line {
            occ += (cell == b'#') as u64;
        }
    }
    return occ;
}

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let cells = aoc2020::read_lines(stdin)?;
    println!("part 1: {}", part1(cells.clone()));
    println!("part 2: {}", part2(cells));
    return Ok(());
}
