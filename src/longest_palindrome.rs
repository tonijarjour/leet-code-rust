use std::collections::HashMap;

pub fn solve(s: String) -> i32 {
    let mut map = HashMap::new();

    for c in s.chars() {
        match map.get_mut(&c) {
            Some(v) => {
                *v += 1;
            }
            None => {
                map.insert(c, 1);
            }
        }
    }

    let mut len = 0;
    let mut odd = false;

    for v in map.values() {
        if v % 2 == 0 {
            len += v;
        } else {
            odd = true;
            len += v - 1;
        }
    }

    if odd {
        return len + 1;
    }

    len
}
