pub fn run(input: &[u8]) -> u32 {
    unsafe {
        let line_width = input.iter().position(|&b| b == b'\n').unwrap() + 1;

        let mut prev = vec![false; line_width];
        let center = input.iter().position(|&b| b == b'S').unwrap();
        prev[center] = true;

        let mut swap = prev.clone();

        let mut out = 0;
        (line_width * 2..input.len()).step_by(line_width * 2).enumerate().for_each(
            |(line_no, line_start)| {
                let start = center - line_no - 1;
                let end = center + 2 + line_no;
                for col in start..end {
                    if *prev.get_unchecked(col) {
                        let has_split = input.get_unchecked(line_start + col) == &b'^';
                        if has_split {
                            out += 1;
                            *swap.get_unchecked_mut(col - 1) = true;
                            *swap.get_unchecked_mut(col) = false;
                            *swap.get_unchecked_mut(col + 1) = true;
                        }
                    }
                }
                prev.copy_from_slice(&swap);
            },
        );

        out
    }
}
