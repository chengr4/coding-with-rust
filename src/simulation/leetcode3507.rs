pub fn minimum_pair_removal(mut nums: Vec<i32>) -> i32 {
    let mut steps = 0;

    loop {
        // Check if sorted
        let mut is_sorted = true;
        for i in 0..nums.len().saturating_sub(1) {
            if nums[i] > nums[i + 1] {
                is_sorted = false;
                break;
            }
        }

        if is_sorted {
            return steps;
        }

        let mut min_sum = i32::MAX;
        let mut target_index = 0;

        for i in 0..nums.len().saturating_sub(1) {
            let current_sum = nums[i] + nums[i + 1];
            if current_sum < min_sum {
                min_sum = current_sum;
                target_index = i;
            }
        }

        nums[target_index] = min_sum;
        nums.remove(target_index + 1);

        steps += 1;
    }
}
