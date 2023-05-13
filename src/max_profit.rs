pub fn solve(prices: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut low = prices[0];

    for n in prices {
        low = i32::min(low, n);
        max = i32::max(max, n - low);
    }

    max
}
