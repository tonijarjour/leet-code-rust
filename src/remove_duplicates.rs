pub fn solve(nums: &mut Vec<i32>) -> i32 {
    nums.dedup();

    nums.len() as i32
}

pub fn run() {
    let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    println!("{}", solve(&mut nums));
}
