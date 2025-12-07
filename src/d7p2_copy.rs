pub fn run(input: &[u8]) -> u64 {
    unsafe {
        let line_width = input.iter().position(|&b| b == b'\n').unwrap() + 1;

        let mut buf = vec![0u64; line_width];
        let center = input.iter().position(|&b| b == b'S').unwrap();
        buf[center] = 1;

        let mut swap = buf.clone();

        (line_width * 2..input.len()).step_by(line_width * 2).enumerate().for_each(
            |(line_no, line_start)| {
                let start = center - line_no - 1;
                let end = center + 2 + line_no;
                for col in start..end {
                    let &paths = buf.get_unchecked(col);
                    let has_split = input.get_unchecked(line_start + col) == &b'^';
                    if has_split {
                        *swap.get_unchecked_mut(col - 1) += paths;
                        *swap.get_unchecked_mut(col) = 0;
                        *swap.get_unchecked_mut(col + 1) += paths;
                    }
                }
                buf.copy_from_slice(&swap);
            },
        );

        buf.iter().sum()
    }
}
