use std::collections::HashSet;

pub fn solve(nums: &mut Vec<i32>) -> i32 {
    let mut set = HashSet::new();

    let mut n = 0;
    while n < nums.len() {
        if set.contains(&nums[n]) {
            nums.remove(n);
        } else {
            set.insert(nums[n]);
            n += 1
        }
    }

    nums.len() as i32
}

pub fn run() {
    let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    println!("{}", solve(&mut nums));
}
