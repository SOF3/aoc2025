use std::{fmt, ops};

pub fn run(input: &[u8]) -> u32 {
    match input.iter().position(|&b| b == b'\n') {
        Some(15) => run_generic::<15, _>(input),
        Some(100) => run_generic::<100, _>(input),
        _ => unreachable!(),
    }
}

fn run_generic<
    const N: usize,
    Sum: Default + ops::AddAssign + ops::Mul<Output = Sum> + Pow10 + From<u8> + fmt::Debug,
>(
    input: &[u8],
) -> Sum {
    unsafe {
        const WANT_DIGITS: usize = 2;

        let mut sum = Sum::default();
        for start_offset in (0..input.len()).step_by(N + 1) {
            let end_offset = start_offset + N;
            let mut line = input.get_unchecked(start_offset..end_offset);

            let mut next_needle = b'9';
            let mut remaining_want = WANT_DIGITS - 1;
            loop {
                let haystack_len = line.len() - remaining_want;
                match line.get_unchecked(..haystack_len).iter().position(|&b| b == next_needle) {
                    None if next_needle > b'0' => {
                        next_needle -= 1;
                    }
                    None => break,
                    Some(pos) => {
                        let byte = line[pos];
                        sum += Sum::pow10(remaining_want) * Sum::from(byte - b'0');
                        if remaining_want == 0 {
                            break;
                        }
                        remaining_want -= 1;

                        // assumption: line[..haystack_len].all(|b| b <= next_needle)
                        let &next_byte = line.get_unchecked(haystack_len);
                        next_needle = next_byte.max(next_needle);

                        line = line.get_unchecked(pos + 1..);
                    }
                }
            }
        }
        sum
    }
}

trait Pow10 {
    fn pow10(n: usize) -> Self;
}

impl Pow10 for u16 {
    fn pow10(n: usize) -> Self {
        match n {
            0 => 1,
            1 => 10,
            2 => 100,
            3 => 1_000,
            4 => 10_000,
            _ => unreachable!(),
        }
    }
}

impl Pow10 for u32 {
    fn pow10(n: usize) -> Self {
        match n {
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
            _ => unreachable!(),
        }
    }
}

impl Pow10 for u64 {
    fn pow10(n: usize) -> Self {
        match n {
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
}
