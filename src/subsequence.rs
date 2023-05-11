pub fn solve(s: String, t: String) -> bool {
    if s.len() > t.len() || (s.len() == t.len() && s != t) {
        return false;
    }

    let mut pos = 0;
    for c in 0..s.len() {
        let mut found = false;
        for n in pos..t.len() {
            if s.as_bytes()[c] == t.as_bytes()[n] {
                found = true;
                pos = n + 1;
                break;
            }
        }
        if !found {
            return false;
        }
    }

    true
}
