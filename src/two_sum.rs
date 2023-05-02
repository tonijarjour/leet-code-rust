use std::collections::HashMap;

pub fn solve(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut res: Vec<i32> = Vec::new();

    for (i, n) in nums.into_iter().enumerate() {
        match map.get(&(target - n)) {
            Some(f) => {
                res.push(*f);
                res.push(i as i32);
                break;
            }
            None => {
                map.insert(n, i as i32);
            }
        }
    }

    res
}
