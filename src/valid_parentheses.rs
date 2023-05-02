pub fn solve(s: String) -> bool {
    let mut stack = Vec::new();

    for c in s.chars() {
        if "[({".contains(c) {
            stack.push(c);
        } else if let Some(o) = stack.pop() {
            match (o, c) {
                ('[', ']') | ('{', '}') | ('(', ')') => (),
                _ => return false,
            }
        } else {
            return false;
        }
    }

    stack.is_empty()
}
