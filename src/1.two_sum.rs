#![allow(unused)]

use std::collections::HashMap;

#[test]
fn test_two_sum() {
    two_sum(vec![2, 7, 11, 15], 9);
}
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut cur: &i32;
    let mut div: i32;

    for (idx, v) in nums.iter().enumerate() {
        cur = &(nums[idx]);
        div = target - v;

        if map.contains_key(&div) {
            return  vec![*map.get(&div).unwrap(), idx as i32];
        }
        map.insert(*cur, idx as i32);
    }
    vec![]
}
