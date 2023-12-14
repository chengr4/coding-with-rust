use std::vec;

pub fn min_distance(word1: String, word2: String) -> i32 {
    let mut dp_matrix = vec![vec![0;word2.len()+1];word1.len()+1];
    let word1_chars: Vec<char> = word1.chars().collect();
    let word2_chars: Vec<char> = word2.chars().collect();

    for i in (0..word1.len()).rev() {
        dp_matrix[i][word2.len()] = dp_matrix[i+1][word2.len()] + 1;
    }

    for j in (0..word2.len()).rev() {
        dp_matrix[word1.len()][j] = dp_matrix[word1.len()][j+1] + 1;
    }

    for row in (0..word1.len()).rev() {
        for col in (0..word2.len()).rev() {
            if word1_chars[row] == word2_chars[col] {
                dp_matrix[row][col] = dp_matrix[row+1][col+1];
            } else {
                dp_matrix[row][col] = (dp_matrix[row+1][col+1]+1).min((dp_matrix[row+1][col]+1).min(dp_matrix[row][col+1]+1));
            }
        }
    }

    dp_matrix[0][0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_distance() {
        assert_eq!(min_distance("horse".to_string(), "ros".to_string()), 3);
        assert_eq!(min_distance("intention".to_string(), "execution".to_string()), 5);
        assert_eq!(min_distance("".to_string(), "e".to_string()), 1);
    }
}