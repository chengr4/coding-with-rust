pub fn max_area(height: Vec<i32>) -> i32 {
    let mut res = 0;
    let mut l = 0;
    let mut r = height.len() - 1;

    while l < r {
        let curr_area = (r - l) as i32 * height[l].min(height[r]);
        res = res.max(curr_area);

        if height[l] <= height[r] {
            l += 1;
        } else {
            r -= 1;
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_11() {
        assert_eq!(max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(max_area(vec![1, 1]), 1);
    }
}
