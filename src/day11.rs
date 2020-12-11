use std::io;

const DIRS: &[(isize, isize)] = &[
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn is_occupied(
    cells: &Vec<Vec<u8>>,
    mut i: isize,
    mut j: isize,
    di: isize,
    dj: isize,
    max_radius: bool,
) -> u64 {
    let rows = cells.len() as isize;
    let cols = cells[0].len() as isize;
    loop {
        i += di;
        j += dj;

        if i < 0 || i >= rows || j < 0 || j >= cols {
            break;
        }

        match cells[i as usize][j as usize] {
            b'.' => {}
            b'#' => return 1,
            b'L' => return 0,
            _ => panic!("wtf"),
        }

        if !max_radius {
            break;
        }
    }

    return 0;
}

fn unified_parts(cells: Vec<Vec<u8>>, too_many: u64, max_radius: bool) -> u64 {
    let mut cells = cells.clone();
    let mut changed = true;
    let rows = cells.len();
    let cols = cells[0].len();

    while changed {
        let mut new_cells = cells.clone();
        for i in 0..rows {
            for j in 0..cols {
                if cells[i][j] == b'.' {
                    continue;
                }
                let mut occupied = 0;
                for (di, dj) in DIRS {
                    occupied += is_occupied(&cells, i as isize, j as isize, *di, *dj, max_radius);
                }
                if occupied >= too_many {
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
    println!("part 1: {}", unified_parts(cells.clone(), 4, false));
    println!("part 2: {}", unified_parts(cells, 5, true));
    return Ok(());
}
