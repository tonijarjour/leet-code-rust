use std::collections::{HashMap, HashSet};

pub fn solve(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut map = HashMap::new();
    let mut set = HashSet::new();

    for n in 0..s.len() {
        if let Some(c) = map.get(&s.as_bytes()[n]) {
            if c != &t.as_bytes()[n] {
                return false;
            }
        } else {
            if set.contains(&t.as_bytes()[n]) {
                return false;
            }
            map.insert(s.as_bytes()[n], t.as_bytes()[n]);
            set.insert(t.as_bytes()[n]);
        }
    }

    true
}
