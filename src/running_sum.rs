pub fn solve(nums: Vec<i32>) -> Vec<i32> {
    let mut sum = 0;
    nums.iter()
        .map(|n| {
            sum += n;
            sum
        })
        .collect()
}
