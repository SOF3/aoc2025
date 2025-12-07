pub fn run(input: &[u8]) -> u64 {
    let line_width = input.iter().position(|&b| b == b'\n').unwrap() + 1;

    let mut buf = vec![0u64; line_width];
    let center = input.iter().position(|&b| b == b'S').unwrap();
    buf[center] = 1;

    let mut swap = vec![0u64; line_width];

    (line_width * 2..input.len()).step_by(line_width * 2).enumerate().for_each(
        |(line_no, line_start)| {
            swap.fill(0);
            let start = center - line_no - 1;
            let end = center + 2 + line_no;
            for col in start..end {
                if let paths @ 1.. = buf[col] {
                    let has_split = input[line_start + col] == b'^';
                    if has_split {
                        swap[col - 1] += paths;
                        swap[col + 1] += paths;
                    } else {
                        swap[col] += paths;
                    }
                }
            }
            buf.copy_from_slice(&swap);
        },
    );

    buf.iter().sum()
}
