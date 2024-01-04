pub fn decode_string(s: String) -> String {
    let mut stack: Vec<String> = vec![];

    for ch in s.chars() {
        if ch == ']' {
            let mut curr_string = String::new();
            loop {
                if let Some(curr) = stack.pop() {
                    if curr == '['.to_string() {
                        let mut times = String::new();
                        while let Some(top) = stack.last() {
                            if let Ok(num) = top.parse::<i32>() {
                                times = format!("{}{}", num, times);
                                stack.pop();
                            } else {
                                break;
                            }
                        }
                        curr_string = curr_string.repeat(times.parse().unwrap());
                        stack.push(curr_string);
                        break;
                    }

                    curr_string = curr + &curr_string;
                }
            }
        } else {
            stack.push(ch.to_string());
        }
    }
    stack.concat()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_394() {
        assert_eq!(
            decode_string("3[a2[c]]".to_string()),
            "accaccacc".to_string()
        );
        assert_eq!(
            decode_string("3[a]2[bc]".to_string()),
            "aaabcbc".to_string()
        );
        assert_eq!(
            decode_string("100[leetcode]".to_string()),
            "leetcode".repeat(100)
        );
        assert_eq!(
            decode_string("2[abc]3[cd]ef".to_string()),
            "abcabccdcdcdef".to_string()
        );
        assert_eq!(
            decode_string("abc3[cd]xyz".to_string()),
            "abccdcdcdxyz".to_string()
        );
    }
}
