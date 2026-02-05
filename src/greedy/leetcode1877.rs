pub fn min_pair_sum(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();

    let n  = nums.len();
    let mut max_pair_sum = 0;

    for i in 0..n / 2 {
        let current_sum = nums[i] + nums[n - 1 - i];
        max_pair_sum = max_pair_sum.max(current_sum);
    }

    max_pair_sum
}
