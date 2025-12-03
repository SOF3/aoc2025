pub fn run(s: &[u8]) -> u64 {
    s.strip_suffix(b"\n").unwrap_or(s).split(|&b| b == b',').map(solve_once).sum()
}

fn parse_int(s: &[u8], delim: impl Fn(u8) -> bool) -> (usize, u64) {
    let mut sum = 0;
    let mut chars = 0;
    for &b in s {
        if delim(b) {
            break;
        }
        chars += 1;
        sum = sum * 10 + u64::from(b - b'0');
    }
    (chars, sum)
}

fn solve_once(range: &[u8]) -> u64 {
    let (start_digits, start) = parse_int(range, |b| b == b'-');
    let (end_digits, end) = parse_int(&range[start_digits + 1..], |_| false);

    let mut output = 0;
    for n in start_digits..=end_digits {
        if n % 2 == 0 {
            output += solve_within(n as u32, start, start_digits as u32, end, end_digits as u32);
        }
    }
    output
}

fn solve_within(digits: u32, start: u64, start_digits: u32, end: u64, end_digits: u32) -> u64 {
    let Some((first, last, half)) = get_bounds(digits, start, start_digits, end, end_digits) else {
        return 0;
    };
    gaussian(first, last) * (half + 1)
}

fn get_bounds(
    digits: u32,
    mut start: u64,
    start_digits: u32,
    mut end: u64,
    end_digits: u32,
) -> Option<(u64, u64, u64)> {
    if start_digits < digits {
        start = pow10(digits - 1);
    }
    if end_digits > digits {
        end = pow10(digits) - 1;
    }

    let half = pow10(digits / 2);

    let start_prefix = start / half;
    let start_suffix = start % half;
    let first = if start_prefix < start_suffix { start_prefix + 1 } else { start_prefix };

    let end_prefix = end / half;
    let end_suffix = end % half;
    let last = if end_prefix > end_suffix { end_prefix - 1 } else { end_prefix };

    if last < first {
        return None;
    }

    Some((first, last, half))
}

fn pow10(exp: u32) -> u64 { pow10_match(exp) }

fn pow10_match(exp: u32) -> u64 {
    match exp {
        0 => 1,
        1 => 10,
        2 => 100,
        3 => 1_000,
        4 => 10_000,
        5 => 100_000,
        6 => 1_000_000,
        7 => 10_000_000,
        8 => 100_000_000,
        9 => 1_000_000_000,
        10 => 10_000_000_000,
        11 => 100_000_000_000,
        12 => 1_000_000_000_000,
        13 => 10_000_000_000_000,
        14 => 100_000_000_000_000,
        15 => 1_000_000_000_000_000,
        16 => 10_000_000_000_000_000,
        17 => 100_000_000_000_000_000,
        18 => 1_000_000_000_000_000_000,
        19 => 10_000_000_000_000_000_000,
        _ => unreachable!(),
    }
}

fn gaussian(a: u64, b: u64) -> u64 { (b - a + 1) * (a + b) / 2 }

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_bounds() {
        assert_eq!(super::get_bounds(4, 123, 3, 12345, 5), Some((10, 99, 100)));
        assert_eq!(super::get_bounds(4, 1234, 4, 9876, 4), Some((13, 97, 100)));
        assert_eq!(super::get_bounds(4, 1111, 4, 8888, 4), Some((11, 88, 100)));
        assert_eq!(super::get_bounds(4, 4321, 4, 5678, 4), Some((43, 56, 100)));
        assert_eq!(super::get_bounds(4, 1234, 4, 1235, 4), None);
    }
}
