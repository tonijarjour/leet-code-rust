pub fn solve(strs: Vec<String>) -> String {
    let mut longest = Vec::new();

    'out: for (i, c) in strs[0].as_bytes().iter().enumerate() {
        for s in strs[1..].iter() {
            if i == s.as_bytes().len() || s.as_bytes()[i] != *c {
                break 'out;
            }
        }
        longest.push(*c);
    }

    std::str::from_utf8(&longest).unwrap().to_string()
}
