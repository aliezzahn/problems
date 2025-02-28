use std::collections::VecDeque;


pub struct Solution;


impl Solution {
    pub fn strong_password_checker(password: String) -> i32 {
        let n = password.len();
        let mut missing_types = 3;
        let mut has_lower = false;
        let mut has_upper = false;
        let mut has_digit = false;

        let chars: Vec<char> = password.chars().collect();

        // Check for missing character types
        for &c in &chars {
            if !has_lower && c.is_lowercase() {
                has_lower = true;
                missing_types -= 1;
            } else if !has_upper && c.is_uppercase() {
                has_upper = true;
                missing_types -= 1;
            } else if !has_digit && c.is_digit(10) {
                has_digit = true;
                missing_types -= 1;
            }
        }

        let mut replace = 0;
        let mut one_seq = 0;
        let mut two_seq = 0;
        let mut i = 2;

        // Check for repeating sequences
        while i < n {
            if chars[i] == chars[i - 1] && chars[i - 1] == chars[i - 2] {
                let mut length = 2;
                while i < n && chars[i] == chars[i - 1] {
                    length += 1;
                    i += 1;
                }
                replace += length / 3;
                if length % 3 == 0 {
                    one_seq += 1;
                } else if length % 3 == 1 {
                    two_seq += 1;
                }
            } else {
                i += 1;
            }
        }

        // Calculate the minimum number of changes
        if n < 6 {
            return std::cmp::max(missing_types, 6 - n as i32);
        } else if n <= 20 {
            return std::cmp::max(missing_types, replace);
        } else {
            let delete_count = n - 20;
            replace -= std::cmp::min(delete_count, one_seq * 1) as i32 / 1;
            replace -= std::cmp::min(std::cmp::max(delete_count - one_seq, 0), two_seq * 2) as i32 / 2;
            replace -= std::cmp::max(delete_count - one_seq - 2 * two_seq, 0) as i32 / 3;
            return delete_count as i32 + std::cmp::max(missing_types, replace);
        }
    }

    pub fn shortest_common_supersequence(str1: String, str2: String) -> String {
        let m = str1.len();
        let n = str2.len();
        let str1 = str1.as_bytes();
        let str2 = str2.as_bytes();

        // Step 1: Find the longest common subsequence using dynamic programming
        let mut dp = vec![vec![0; n + 1]; m + 1];

        // Fill the dp table
        for i in 1..=m {
            for j in 1..=n {
                if str1[i - 1] == str2[j - 1] {
                    dp[i][j] = 1 + dp[i - 1][j - 1];
                } else {
                    dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
                }
            }
        }

        // Step 2: Construct the shortest common supersequence
        let mut i = m;
        let mut j = n;
        let mut result = VecDeque::new();

        while i > 0 && j > 0 {
            if str1[i - 1] == str2[j - 1] {
                // If the characters are the same, add it once
                result.push_front(str1[i - 1]);
                i -= 1;
                j -= 1;
            } else if dp[i - 1][j] > dp[i][j - 1] {
                // If coming from top has higher value, take character from str1
                result.push_front(str1[i - 1]);
                i -= 1;
            } else {
                // Otherwise, take character from str2
                result.push_front(str2[j - 1]);
                j -= 1;
            }
        }

        // Add remaining characters from str1 (if any)
        while i > 0 {
            result.push_front(str1[i - 1]);
            i -= 1;
        }

        // Add remaining characters from str2 (if any)
        while j > 0 {
            result.push_front(str2[j - 1]);
            j -= 1;
        }

        // Convert result from VecDeque<u8> to String
        unsafe { String::from_utf8_unchecked(result.into()) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}

