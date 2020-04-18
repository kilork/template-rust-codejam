use std::io::{BufRead, BufReader};
use std::io::prelude::*;

// dbg!
// rustc 1.24.0

fn main() {
    let stdio = std::io::stdin();
    let input = BufReader::new(stdio.lock());
    let mut stdout = std::io::stdout();
    process_cases(input, &mut stdout);
}

fn process_case<R: std::io::BufRead + Read>(mut r: R) -> u64 {
    let n = read_usize(&mut r);
    let mut sum = 0;
    for _ in 0..n {
        let line_sum: u64 = read_vec(&mut r).iter().sum();
        sum += line_sum;
    }
    sum
}

fn process_cases<R: std::io::BufRead + Read, W: Write>(mut r: R, w: &mut W) {
    let n = read_usize(&mut r);

    for i in 0..n {
        let result = process_case(&mut r);
        writeln!(w, "Case #{}: {}", i + 1, result).unwrap();
    }
}

fn read_usize<R: Read + BufRead>(r: &mut R) -> usize {
    read_line(r).parse().unwrap()
}

fn read_vec<R: Read + BufRead>(r: &mut R) -> Vec<u64> {
    read_line(r)
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn read_vecs<R: Read + BufRead>(r: &mut R) -> Vec<String> {
    read_line(r).split_whitespace().map(|x| x.into()).collect()
}

fn read_line<R: Read + BufRead>(r: &mut R) -> String {
    let mut result = String::new();
    r.read_line(&mut result).unwrap();
    result.trim_end().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        let input = &include_bytes!("input.txt")[..];
        let expected_output = include_bytes!("output.txt");
        let mut output: Vec<u8> = vec![];
        process_cases(input, &mut output);
        assert_eq!(
            String::from_utf8_lossy(expected_output),
            String::from_utf8_lossy(&output)
        );
    }
}
