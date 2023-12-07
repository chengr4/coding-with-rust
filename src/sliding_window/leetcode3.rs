use std::collections::HashSet;


pub fn length_of_longest_substring(s: String) -> i32 {
    let mut set = HashSet::new();
    let mut res = 0;
    let mut curr_max = 0;

    let mut l = 0;
    for ch in s.chars() {
        while set.contains(&ch) {
            set.remove(&(s.as_bytes()[l] as char));
            l +=1;
            curr_max -= 1;
        }

        curr_max += 1;
        set.insert(ch);
        res = res.max(curr_max);
    }

    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_length_of_longest_substring() {
        assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
        assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);
        assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3);
        assert_eq!(length_of_longest_substring("".to_string()), 0);
    }
}






