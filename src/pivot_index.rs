pub fn pivot_index(nums: Vec<i32>) -> i32 {
    let mut left_sum = 0;
    let mut right_sum = nums.iter().skip(1).sum();

    if left_sum == right_sum {
        return 0;
    }

    for i in 1..nums.len() {
        left_sum += nums[i - 1];
        right_sum -= nums[i];

        if left_sum == right_sum {
            return i as i32;
        }
    }

    return -1;
}
