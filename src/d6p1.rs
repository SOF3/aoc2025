use std::hint::unreachable_unchecked;

pub fn run(input: &[u8]) -> u64 {
    let line_width = input.iter().position(|&b| b == b'\n').unwrap() + 1;
    let num_lines = input.len() / line_width - 1;
    match num_lines {
        3 => solve_p1::<3>(input, line_width),
        4 => solve_p1::<4>(input, line_width),
        _ => unreachable!(),
    }
}

fn solve_p1<const NUM_LINES: usize>(input: &[u8], line_width: usize) -> u64 {
    let mut ops = &input[NUM_LINES * line_width..];
    let mut col_offset = 0;

    let mut output = 0;
    loop {
        let op = ops[0];
        let op_width = match ops[1..].iter().position(|&b| b != b' ') {
            Some(pos) => pos + 1,
            None => ops.len(),
        };
        ops = &ops[op_width..];

        match op {
            b'+' => do_add::<NUM_LINES>(input, line_width, col_offset, op_width, &mut output),
            b'*' => do_mul::<NUM_LINES>(input, line_width, col_offset, op_width, &mut output),
            b'\n' => break,
            _ => unsafe { unreachable_unchecked() },
        }

        col_offset += op_width;
    }
    output
}

fn do_add<const NUM_LINES: usize>(
    input: &[u8],
    line_width: usize,
    col_offset: usize,
    op_width: usize,
    output: &mut u64,
) {
    for i in 0..NUM_LINES {
        let val_str =
            unsafe { input.get_unchecked(line_width * i + col_offset..).get_unchecked(..op_width) };
        let mut val = 0u64;

        for &b in val_str {
            if b >= b'0' {
                val *= 10;
                val += u64::from(b) - u64::from(b'0');
            }
        }

        *output += val;
    }
}

fn do_mul<const NUM_LINES: usize>(
    input: &[u8],
    line_width: usize,
    col_offset: usize,
    op_width: usize,
    output: &mut u64,
) {
    let mut prod: u64 = 1;
    for i in 0..NUM_LINES {
        let val_str =
            unsafe { input.get_unchecked(line_width * i + col_offset..).get_unchecked(..op_width) };
        let mut val = 0u64;

        for &b in val_str {
            if b >= b'0' {
                val *= 10;
                val += u64::from(b) - u64::from(b'0');
            }
        }

        prod *= val;
    }

    *output += prod;
}
