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

pub fn read_lines<R: io::BufRead>(r: R) -> anyhow::Result<Vec<Vec<u8>>> {
    let mut lines: Vec<Vec<u8>> = Vec::new();
    for line in r.lines() {
        let line = line?;
        lines.push(line.into_bytes());
    }
    return Ok(lines);
}
