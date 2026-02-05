pub fn construct_transformed_array(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut res = vec![0; n];

    for i in 0..n {
        let to_move = nums[i];

        if to_move == 0 {
            res[i] = nums[i];
        } else {
            let new_pos = (i as i32 + to_move).rem_euclid(n as i32) as usize;
            res[i] += nums[new_pos];
        }
    }

    res
}
