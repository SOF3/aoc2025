pub fn run(mut s: &[u8]) -> u32 {
    let mut sum = 50i32;
    let mut cnt = 0;

    while let Some(&dir) = s.split_off_first() {
        match dir {
            b'L' => {
                run_once(&mut s, &mut sum, &mut cnt, |a, b| *a -= b);
            }
            b'R' => {
                run_once(&mut s, &mut sum, &mut cnt, |a, b| *a += b);
            }
            _ => unreachable!(),
        }
    }
    cnt
}

fn parse_once(s: &[u8]) -> (usize, i32) {
    if s.get(1) == Some(&b'\n') {
        (2, (s[0] - b'0').into())
    } else if s.get(2) == Some(&b'\n') {
        (3, i32::from(s[0] - b'0') * 10 + i32::from(s[1] - b'0'))
    } else if s.get(3) == Some(&b'\n') {
        (4, i32::from(s[1] - b'0') * 10 + i32::from(s[2] - b'0'))
    } else {
        unreachable!()
    }
}

fn run_once(s: &mut &[u8], sum: &mut i32, cnt: &mut u32, f: impl Fn(&mut i32, i32)) {
    let (len, v) = parse_once(s);
    *s = &s[len..];
    f(sum, v);
    if *sum % 100 == 0 {
        *cnt += 1;
    }
}
