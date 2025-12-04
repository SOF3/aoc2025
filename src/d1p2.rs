use std::hint::unreachable_unchecked;

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
            _ => unsafe { unreachable_unchecked() },
        }
    }
    cnt
}

fn parse_once(s: &[u8]) -> (usize, u8, i32) {
    if s.get(1) == Some(&b'\n') {
        (2, 0, (s[0] - b'0').into())
    } else if s.get(2) == Some(&b'\n') {
        (3, 0, i32::from(s[0] - b'0') * 10 + i32::from(s[1] - b'0'))
    } else if s.get(3) == Some(&b'\n') {
        (4, s[0] - b'0', i32::from(s[1] - b'0') * 10 + i32::from(s[2] - b'0'))
    } else {
        unsafe { unreachable_unchecked() }
    }
}

fn run_once(s: &mut &[u8], sum: &mut i32, cnt: &mut u32, f: impl Fn(&mut i32, i32)) {
    let (len, excess, v) = parse_once(s);
    *cnt += u32::from(excess);
    *s = &s[len..];
    let was_zero = *sum == 0;
    f(sum, v);
    if *sum >= 100 {
        *sum -= 100;
        *cnt += 1;
    } else if *sum < 0 {
        *sum += 100;
        if !was_zero {
            *cnt += 1;
        }
    } else if *sum == 0 {
        *cnt += 1;
    }
}
