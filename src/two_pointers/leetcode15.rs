use std::collections::HashSet;

pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = Vec::new();
    let mut saved = HashSet::new();
    nums.sort();

    for i in 1..(nums.len() - 1) {
        let mut l = i - 1;
        let mut r = i + 1;
        while r < nums.len() {
            let curr_sum = nums[l] + nums[i] + nums[r];
            if saved.contains(&vec![nums[l], nums[i], nums[r]]) {
                break;
            }

            if curr_sum < 0 {
                r += 1;
            } else if curr_sum > 0 {
                if l == 0 {
                    break;
                }
                l -= 1;
            } else {
                res.push(vec![nums[l], nums[i], nums[r]]);
                saved.insert(vec![nums[l], nums[i], nums[r]]);
                r += 1;

                while r < nums.len() && nums[r] == nums[r - 1] {
                    r += 1;
                }
            }
        }
    }

    res
}

// -4 -1 -1 0 1 2

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_15() {
        assert_eq!(
            three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        );
        assert_eq!(three_sum(vec![0, 1, 1]), vec![] as Vec<Vec<i32>>);
        assert_eq!(three_sum(vec![0, 0, 0]), vec![vec![0, 0, 0]]);
        assert_eq!(three_sum(vec![0, 0, 0, 0]), vec![vec![0, 0, 0]]);
    }
}
