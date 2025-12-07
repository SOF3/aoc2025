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

        input = input.get_unchecked(1..);

        let mut points = Vec::with_capacity(1024);
        while !input.is_empty() {
            let (point, n) = parse_int_until(input, b'\n');
            points.push(point);
            input = input.get_unchecked(n + 1..);
        }

        points.sort_unstable();

        let mut ranges = &ranges[..];
        let mut points = &points[..];

        let mut out = 0;
        while let (Some(&point), Some(&(start, end))) = (points.first(), ranges.first()) {
            if point < start {
                points = &points[1..];
            } else if point <= end {
                out += 1;
                points = &points[1..];
            } else {
                ranges = &ranges[1..];
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
