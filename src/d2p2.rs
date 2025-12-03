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
        for factor in factorize(n) {
            let sum = solve_within(
                n,
                factor.part_len,
                factor.part_cnt,
                start,
                start_digits,
                end,
                end_digits,
            );
            if factor.negate {
                output -= sum;
            } else {
                output += sum;
            }
        }
    }
    output
}

fn solve_within(
    digits: usize,
    part_len: usize,
    part_cnt: usize,
    start: u64,
    start_digits: usize,
    end: u64,
    end_digits: usize,
) -> u64 {
    let Some((first, last)) =
        get_bounds(digits, part_len, part_cnt, start, start_digits, end, end_digits)
    else {
        return 0;
    };
    gaussian(first, last) * splat(1, part_len, part_cnt)
}

fn get_bounds(
    digits: usize,
    part_len: usize,
    part_cnt: usize,
    mut start: u64,
    start_digits: usize,
    mut end: u64,
    end_digits: usize,
) -> Option<(u64, u64)> {
    if start_digits < digits {
        start = pow10(digits - 1);
    }
    if end_digits > digits {
        end = pow10(digits) - 1;
    }

    let prefix_div = pow10(part_len * (part_cnt - 1));

    let start_prefix = start / prefix_div;
    let mut first_elem = start_prefix;
    if splat(start_prefix, part_len, part_cnt) < start {
        first_elem += 1;
    }

    let end_prefix = end / prefix_div;
    let mut last_elem = end_prefix;
    if splat(end_prefix, part_len, part_cnt) > end {
        last_elem -= 1;
    }

    if first_elem > last_elem {
        return None;
    }

    Some((first_elem, last_elem))
}

fn splat(substr: u64, len: usize, cnt: usize) -> u64 {
    let mut result = 0;
    for i in 0..cnt {
        result += substr * pow10(len * i);
    }
    result
}

fn pow10(exp: usize) -> u64 { pow10_match(exp as u32) }

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

fn factorize(digits: usize) -> &'static [Factor] {
    match digits {
        1 => &[],
        2 => &[Factor { part_len: 1, part_cnt: 2, negate: false }],
        3 => &[Factor { part_len: 1, part_cnt: 3, negate: false }],
        4 => &[Factor { part_len: 2, part_cnt: 2, negate: false }],
        5 => &[Factor { part_len: 1, part_cnt: 5, negate: false }],
        6 => &[
            Factor { part_len: 2, part_cnt: 3, negate: false },
            Factor { part_len: 3, part_cnt: 2, negate: false },
            Factor { part_len: 1, part_cnt: 6, negate: true },
        ],
        7 => &[Factor { part_len: 1, part_cnt: 7, negate: false }],
        8 => &[Factor { part_len: 4, part_cnt: 2, negate: false }],
        9 => &[Factor { part_len: 3, part_cnt: 3, negate: false }],
        10 => &[
            Factor { part_len: 2, part_cnt: 5, negate: false },
            Factor { part_len: 5, part_cnt: 2, negate: false },
            Factor { part_len: 1, part_cnt: 10, negate: true },
        ],
        _ => unreachable!(),
    }
}

#[derive(Debug)]
struct Factor {
    part_len: usize,
    part_cnt: usize,
    negate:   bool,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_splat() {
        assert_eq!(super::splat(12, 2, 3), 121212);
        assert_eq!(super::splat(3, 1, 5), 33333);
        assert_eq!(super::splat(444444, 6, 1), 444444);
    }

    #[test]
    fn test_get_bounds() {
        assert_eq!(super::get_bounds(4, 2, 2, 123, 3, 12345, 5), Some((10, 99)));
        assert_eq!(super::get_bounds(4, 2, 2, 1234, 4, 9876, 4), Some((13, 97)));
        assert_eq!(super::get_bounds(4, 2, 2, 1111, 4, 8888, 4), Some((11, 88)));
        assert_eq!(super::get_bounds(4, 2, 2, 4321, 4, 5678, 4), Some((43, 56)));
        assert_eq!(super::get_bounds(4, 2, 2, 1234, 4, 1235, 4), None);
    }
}
