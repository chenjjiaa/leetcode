use std::collections::HashSet;

#[allow(unused)]
pub fn length_of_longest_substring(s: String) -> i32 {
    if s.len() == 0 || s.len() == 1 {
        return  s.len() as i32;
    }
    let mut l = 0;
    let mut r = 0;
    let mut max = 0;
    let charts = &s.chars().collect::<Vec<char>>();
    let mut set: HashSet<char> = HashSet::new();

    while r < s.len() {
        if !set.contains(&charts[r]) {
            set.insert(charts[r]);
            let len = set.len();
            max = if len > max { len } else { max };
            r += 1;
        } else {
            while set.contains(&charts[r]) {
                set.remove(&charts[l]);
                l += 1;
            }
            set.insert(charts[r]);
            r += 1;
        }
    }

    max as i32
}
