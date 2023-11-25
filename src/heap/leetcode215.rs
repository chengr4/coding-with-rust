use std::collections::BinaryHeap;

pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    let mut max_heap = BinaryHeap::from(nums);

    for _ in 0..k-1 {
        max_heap.pop();
    }

    *max_heap.peek().unwrap()
}

// test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_kth_largest() {
        let nums = vec![3, 2, 1, 5, 6, 4];
        let res = find_kth_largest(nums, 2);
        assert_eq!(res, 5);
    }
}