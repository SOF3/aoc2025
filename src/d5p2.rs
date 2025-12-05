use std::hint::{assert_unchecked, unreachable_unchecked};

pub fn run(mut input: &str) -> u64 {
    unsafe{
        let mut ranges = Vec::with_capacity(200);

        while input.as_bytes()[0] != b'\n' {
            let (start, n) = parse_int_until(input, b'-');
            input = input.get_unchecked(n + 1..);
            let (end, n) = parse_int_until(input, b'\n');
            input = input.get_unchecked(n + 1..);
            ranges.push((start, end));
        }

        ranges.sort_unstable_by_key(|&(start, _)| start);

        let mut out = 0;

        let mut state = *ranges.get_unchecked(0);
        for &(start, end) in ranges.get_unchecked(1..) {
            if start <= state.1 {
                state.1 = state.1.max(end);
            } else {
                out += state.1 - state.0 + 1;
                state = (start, end);
            }
        }

        out + state.1 - state.0 + 1
    }
}

fn parse_int_until(s: &str, delim: u8) -> (u64, usize) {
    let mut out = 0;
    for (i, b) in s.bytes().enumerate() {
        if b == delim {
            return (out, i);
        }
        out = out * 10 + u64::from(b - b'0');
    }
    unsafe { unreachable_unchecked() }
}
