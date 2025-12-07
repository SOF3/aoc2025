use std::hint::unreachable_unchecked;

pub fn run(input: &[u8]) -> u64 {
    let line_width = input.iter().position(|&b| b == b'\n').unwrap() + 1;
    let num_lines = input.len() / line_width - 1;
    match num_lines {
        3 => solve_p2::<3>(input, line_width),
        4 => solve_p2::<4>(input, line_width),
        _ => unreachable!(),
    }
}

fn solve_p2<const NUM_LINES: usize>(input: &[u8], line_width: usize) -> u64 {
    let mut ops = &input[NUM_LINES * line_width..];
    let mut col_offset = 0;

    let mut output = 0;
    loop {
        let op = ops[0];
        if op == b'\n' {
            break;
        }

        let op_width = match ops[1..].iter().position(|&b| b != b' ') {
            Some(pos) => pos + 1,
            None => ops.len(),
        };
        ops = &ops[op_width..];
        let is_last = ops[0] == b'\n';
        let val_width = if is_last { op_width } else { op_width - 1 };

        match op {
            b'+' => do_add::<NUM_LINES>(input, line_width, col_offset, val_width, &mut output),
            b'*' => do_mul::<NUM_LINES>(input, line_width, col_offset, val_width, &mut output),
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
    val_width: usize,
    output: &mut u64,
) {
    for col in 0..val_width {
        let mut val = 0u32;

        for line_no in 0..NUM_LINES {
            let &b = unsafe { input.get_unchecked(line_width * line_no + col_offset + col) };
            if b >= b'0' {
                val *= 10;
                val += u32::from(b - b'0');
            }
        }

        *output += u64::from(val);
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
    for col in 0..op_width {
        let mut val = 0u32;

        for line_no in 0..NUM_LINES {
            let &b = unsafe { input.get_unchecked(line_width * line_no + col_offset + col) };
            if b >= b'0' {
                val *= 10;
                val += u32::from(b - b'0');
            }
        }

        prod *= u64::from(val);
    }

    *output += prod;
}

const fn pow10_match(exp: usize) -> u32 {
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
        _ => unsafe { unreachable_unchecked() },
    }
}
