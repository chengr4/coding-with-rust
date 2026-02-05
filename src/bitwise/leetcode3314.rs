pub fn min_bitwise_array(nums: Vec<i32>) -> Vec<i32> {
    let mut ans = Vec::with_capacity(nums.len());

    for &p in &nums {
        if p == 2{
            ans.push(-1);
        } else {
            let mut found = false;
            for x in 0..p {
                if (x | (x + 1)) == p {
                    ans.push(x);
                    found = true;
                    break;
                }
            }

            if !found {
                ans.push(p - 1);
            }
        }
    }

    ans
}
