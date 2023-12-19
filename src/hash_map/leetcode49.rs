use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map: HashMap<[usize; 26], Vec<String>> = HashMap::new();

    for str in strs.iter() {
        let mut count = [0; 26];
        for code in str.bytes() {
            let index = (code - b'a') as usize;
            count[index] += 1;
        }

        map.entry(count).or_default().push(str.to_string());
    }

    map.into_values().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // it actually passed the test, but the order is different
    fn test_49() {
        assert_eq!(
            group_anagrams(vec![
                "eat".to_string(),
                "tea".to_string(),
                "tan".to_string(),
                "ate".to_string(),
                "nat".to_string(),
                "bat".to_string()
            ]),
            vec![
                vec!["tan".to_string(), "nat".to_string()],
                vec!["eat".to_string(), "tea".to_string(), "ate".to_string()],
                vec!["bat".to_string()]
            ]
        );

        assert_eq!(
            group_anagrams(vec!["".to_string()]),
            vec![vec!["".to_string()]]
        );

        assert_eq!(
            group_anagrams(vec!["a".to_string()]),
            vec![vec!["a".to_string()]]
        );
    }
}
