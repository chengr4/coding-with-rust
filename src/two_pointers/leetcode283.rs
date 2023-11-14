pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut l = 0;

    for r in 0..nums.len() {
        if nums[r] != 0 {
            nums.swap(l, r);
            l +=1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_283() {
        let mut nums = vec![0,1,0,3,12];
        move_zeroes(&mut nums);
        assert_eq!(nums, vec![1,3,12,0,0]);

        let mut nums = vec![0];
        move_zeroes(&mut nums);
        assert_eq!(nums, vec![0]);
    }
}
