/*
 * @lc app=leetcode id=153 lang=kotlin
 *
 * [153] Find Minimum in Rotated Sorted Array
 */

// @lc code=start
class Solution {
    fun findMin(nums: IntArray): Int {
        if (nums[0] < nums[nums.size - 1]) {
            return nums[0]
        }
        return this.findMin(nums, 0, nums.size)
    }

    private tailrec fun findMin(nums: IntArray, start: Int, end: Int): Int {
        when (end - start) {
            1 -> return@findMin nums[0]
            in 2..5 -> return@findMin nums.min()!!
        }
        val mid = (start + end) / 2 // always legit
        return if (nums[mid] > nums[start]) {
            findMin(nums, mid + 1, end)
        } else {
            findMin(nums, start, mid + 1)
        }
    }
}
// @lc code=end

