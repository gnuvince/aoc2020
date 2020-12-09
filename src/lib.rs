use std::io;

pub fn read_numbers<R: io::BufRead>(r: R) -> anyhow::Result<Vec<i32>> {
    let mut numbers: Vec<i32> = Vec::with_capacity(4096);
    for line in r.lines() {
        let line = line?;
        let n: i32 = line.parse::<i32>()?;
        numbers.push(n);
    }
    return Ok(numbers);
}

pub fn read_i64s<R: io::BufRead>(r: R) -> anyhow::Result<Vec<i64>> {
    let mut numbers: Vec<i64> = Vec::with_capacity(4096);
    for line in r.lines() {
        let line = line?;
        let n: i64 = line.parse::<i64>()?;
        numbers.push(n);
    }
    return Ok(numbers);
}

pub fn read_lines<R: io::BufRead>(r: R) -> anyhow::Result<Vec<Vec<u8>>> {
    let mut lines: Vec<Vec<u8>> = Vec::new();
    for line in r.lines() {
        let line = line?;
        lines.push(line.into_bytes());
    }
    return Ok(lines);
}

pub fn read_lines_string<R: io::BufRead>(r: R) -> anyhow::Result<Vec<String>> {
    let mut lines: Vec<String> = Vec::new();
    for line in r.lines() {
        let line = line?;
        lines.push(line);
    }
    return Ok(lines);
}

pub fn read_groups_of_lines<R: io::BufRead>(r: R) -> anyhow::Result<Vec<Vec<String>>> {
    let mut groups: Vec<Vec<String>> = Vec::new();
    let mut lines: Vec<String> = Vec::new();
    for line in r.lines() {
        let line = line?;
        if line.is_empty() {
            groups.push(lines);
            lines = Vec::new();
        } else {
            lines.push(line);
        }
    }
    groups.push(lines);
    return Ok(groups);
}
