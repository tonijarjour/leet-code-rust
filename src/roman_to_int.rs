pub fn solve(s: String) -> i32 {
    let mut prev = 'I';

    s.chars().rfold(0, |acc, c| {
        let delta = match (c, prev) {
            ('I', 'V' | 'X') => -1,
            ('X', 'L' | 'C') => -10,
            ('C', 'D' | 'M') => -100,
            ('M', _) => 1000,
            ('D', _) => 500,
            ('C', _) => 100,
            ('L', _) => 50,
            ('X', _) => 10,
            ('V', _) => 5,
            ('I', _) => 1,
            _ => 0,
        };
        prev = c;
        acc + delta
    })
}
