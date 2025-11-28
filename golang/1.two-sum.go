/*
 * @lc app=leetcode id=1 lang=golang
 *
 * [1] Two Sum
 */

// @lc code=start
func twoSum(nums []int, target int) []int {
	mapping := make(map[int]int)
	var result []int

	for i := 0; i < len(nums); i++ {
		cur := nums[i]
		div := target - cur

		if v, ok := mapping[div]; ok {
			return []int{v, i}
		}

		mapping[cur] = i
	}

	return result
}

// @lc code=end
