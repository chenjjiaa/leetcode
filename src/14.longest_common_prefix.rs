#![allow(unused)]

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return String::new();
    }
    if strs.len() == 1 {
        return strs[0].clone();
    }

    /// collect() 可以用"漩涡鱼"语法来明确表示具体类型
    /// let mut chart_strings = strs.iter().map(|s| s.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    /// 也可以用类型推导来完成
    let mut chart_strings: Vec<Vec<char>> = strs.iter().map(|s| s.chars().collect()).collect();

    for i in 0..chart_strings[0].len() {
        let c = chart_strings[0][i];
        for j in 0..chart_strings.len() {
            if i == chart_strings[j].len() || chart_strings[j][i] != c {
                return strs[0][0..i].to_string();
            }
        }
    }

    strs[0].clone()
}
