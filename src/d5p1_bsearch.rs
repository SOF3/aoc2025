use std::hint::unreachable_unchecked;

pub fn run(mut input: &str) -> u32 {
    unsafe {
        let mut ranges = Vec::with_capacity(200);

        while input.as_bytes()[0] != b'\n' {
            let (start, n) = parse_int_until(input, b'-');
            input = input.get_unchecked(n + 1..);
            let (end, n) = parse_int_until(input, b'\n');
            input = input.get_unchecked(n + 1..);
            ranges.push((start, end));
        }

        ranges.sort_unstable_by_key(|&(start, _)| start);

        let mut merged = Vec::with_capacity(ranges.len());
        let mut state = ranges[0];
        for &(start, end) in &ranges[1..] {
            if start <= state.1 {
                state.1 = state.1.max(end);
            } else {
                merged.push(state);
                state = (start, end);
            }
        }
        merged.push(state);

        input = input.get_unchecked(1..);

        let mut out = 0;

        while !input.is_empty() {
            let (point, n) = parse_int_until(input, b'\n');
            input = input.get_unchecked(n + 1..);

            let pp = merged.partition_point(|&(_, end)| end < point);
            if let Some(&(start, _)) = merged.get(pp)
                && start <= point
            {
                out += 1;
            }
        }

        out
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
