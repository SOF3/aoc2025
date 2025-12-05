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
            let sum = solve_within(n, factor, start, start_digits, end, end_digits);
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
    factor: &PreparedFactor,
    start: u64,
    start_digits: usize,
    end: u64,
    end_digits: usize,
) -> u64 {
    let Some((first, last)) = get_bounds(digits, factor, start, start_digits, end, end_digits)
    else {
        return 0;
    };
    gaussian(first, last) * factor.splat
}

fn get_bounds(
    digits: usize,
    factor: &PreparedFactor,
    start: u64,
    start_digits: usize,
    end: u64,
    end_digits: usize,
) -> Option<(u64, u64)> {
    let prefix_div = factor.prefix_div;

    let first_elem = if start_digits < digits {
        factor.min_prefix
    } else {
        let start_prefix = start / prefix_div;
        let mut first_elem = start_prefix;
        if start_prefix * factor.splat < start {
            first_elem += 1;
        }
        first_elem
    };

    let last_elem = if end_digits > digits {
        factor.max_prefix
    } else {
        let end_prefix = end / prefix_div;
        let mut last_elem = end_prefix;
        if end_prefix * factor.splat > end {
            last_elem -= 1;
        }
        last_elem
    };

    if first_elem > last_elem {
        return None;
    }

    Some((first_elem, last_elem))
}

const fn splat(substr: u64, len: usize, cnt: usize) -> u64 {
    let mut result = 0;
    let mut i = 0u32;
    while i < (cnt as u32) {
        result += substr * pow10_match((len as u32) * i);
        i += 1;
    }
    result
}

const fn pow10(exp: usize) -> u64 { pow10_match(exp as u32) }

const fn pow10_match(exp: u32) -> u64 {
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

fn gaussian(a: u64, b: u64) -> u64 { (b + 1 - a) * (a + b) / 2 }

const fn factorize(digits: usize) -> &'static [PreparedFactor] {
    match digits {
        1 => &[],
        2 => &[const { Factor { part_len: 1, part_cnt: 2, negate: false }.prepare() }],
        3 => &[const { Factor { part_len: 1, part_cnt: 3, negate: false }.prepare() }],
        4 => &[const { Factor { part_len: 2, part_cnt: 2, negate: false }.prepare() }],
        5 => &[const { Factor { part_len: 1, part_cnt: 5, negate: false }.prepare() }],
        6 => &[
            const { Factor { part_len: 2, part_cnt: 3, negate: false }.prepare() },
            const { Factor { part_len: 3, part_cnt: 2, negate: false }.prepare() },
            const { Factor { part_len: 1, part_cnt: 6, negate: true }.prepare() },
        ],
        7 => &[const { Factor { part_len: 1, part_cnt: 7, negate: false }.prepare() }],
        8 => &[const { Factor { part_len: 4, part_cnt: 2, negate: false }.prepare() }],
        9 => &[const { Factor { part_len: 3, part_cnt: 3, negate: false }.prepare() }],
        10 => &[
            const { Factor { part_len: 2, part_cnt: 5, negate: false }.prepare() },
            const { Factor { part_len: 5, part_cnt: 2, negate: false }.prepare() },
            const { Factor { part_len: 1, part_cnt: 10, negate: true }.prepare() },
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

struct PreparedFactor {
    negate:     bool,
    splat:      u64,
    prefix_div: u64,
    min_prefix: u64,
    max_prefix: u64,
}

impl Factor {
    const fn prepare(self) -> PreparedFactor {
        PreparedFactor {
            negate:     self.negate,
            splat:      splat(1, self.part_len, self.part_cnt),
            prefix_div: pow10(self.part_len * (self.part_cnt - 1)),
            min_prefix: pow10(self.part_len - 1),
            max_prefix: pow10(self.part_len) - 1,
        }
    }
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
        assert_eq!(super::get_bounds(4, &super::factorize(2)[0], 123, 3, 12345, 5), Some((10, 99)));
        assert_eq!(super::get_bounds(4, &super::factorize(2)[0], 1234, 4, 9876, 4), Some((13, 97)));
        assert_eq!(super::get_bounds(4, &super::factorize(2)[0], 1111, 4, 8888, 4), Some((11, 88)));
        assert_eq!(super::get_bounds(4, &super::factorize(2)[0], 4321, 4, 5678, 4), Some((43, 56)));
        assert_eq!(super::get_bounds(4, &super::factorize(2)[0], 1234, 4, 1235, 4), None);
    }
}
