/*
 * @lc app=leetcode.cn id=5 lang=rust
 *
 * [5] 最长回文子串
 */

// @lc code=start
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut charts = &s.chars().collect::<Vec<char>>();
        let mut l = 0;
        let mut r = 0;
        let mut max = 0;
        let mut result = String::new();

        for (i, v) in charts.iter().enumerate() {
            // 奇数
            let odd = infection(i as i32, i as i32, charts);
            // 偶数
            let even = infection(i as i32, (i + 1) as i32, charts);

            let (start, end) = if odd.1 - odd.0 > even.1 - even.0 {
                odd
            } else {
                even
            };

            result = if result.len() < end - start { s[start..end].to_string() } else { result };
        }

        result
    }
}

// infection: charts: &[char], Vec[char] 的 deref 可自动转化
pub fn infection(mut l: i32, mut r: i32, charts: &[char]) -> (usize, usize) {
    while l >= 0 && r < charts.len() as i32 && charts[l as usize] == charts[r as usize] {
        l -= 1;
        r += 1;
    }
    ((l + 1) as usize, r as usize)
}

// @lc code=end

