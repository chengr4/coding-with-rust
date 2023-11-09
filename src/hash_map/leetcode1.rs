use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut res: Vec<i32> = vec![];
    let mut map = HashMap::new();

    for (i, v) in nums.iter().enumerate() {
        let remain = target - v;

        if map.contains_key(&remain) {
            res.push(i.try_into().unwrap());
            res.push(*map.get(&remain).unwrap());
        }

        map.insert(v, i.try_into().unwrap());
    }

    res
}

// test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![1, 0]);
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![2, 1]);
        assert_eq!(two_sum(vec![3, 3], 6), vec![1, 0]);
    }
}