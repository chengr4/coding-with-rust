pub fn climb_stairs(n: i32) -> i32 {
    let usize_n: usize = n.try_into().unwrap();
    let mut dp = vec![0; usize_n+1];
    if usize_n <= 2 {
        return n;
    }
    dp[1] = 1;
    dp[2] = 2;

    for e in 3..usize_n+1 {
        dp[e] = dp[e-1] + dp[e-2];
    }

    dp[usize_n]
}

// test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_climb_stairs() {
        assert_eq!(climb_stairs(1), 1);
        assert_eq!(climb_stairs(2), 2);
        assert_eq!(climb_stairs(3), 3);
        assert_eq!(climb_stairs(6), 13);
    }
}
