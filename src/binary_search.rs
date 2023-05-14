pub fn solve(nums: Vec<i32>, target: i32) -> i32 {
        let mut left: i32 = 0;
        let mut right: i32 = (nums.len() - 1) as i32;

        while left <= right {
            let mid = (left + right) / 2;

            match nums[mid as usize].cmp(&target) {
                std::cmp::Ordering::Equal => return mid,
                std::cmp::Ordering::Less => left = mid + 1,
                std::cmp::Ordering::Greater => right = mid - 1,
            }
        }

        -1
}
