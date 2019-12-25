/*
 * @lc app=leetcode id=154 lang=kotlin
 *
 * [154] Find Minimum in Rotated Sorted Array II
 */

// @lc code=start
import kotlin.math.min

class Solution {
    fun findMin(nums: IntArray): Int {
        if (nums[0] < nums[nums.size - 1]) {
            return nums[0]
        }
        return this.findMin(nums, 0, nums.size)
    }

    private tailrec fun findMin(nums: IntArray, start: Int, end: Int): Int {
        when (end - start) {
            1 -> return@findMin nums[start]
            in 2..10 -> return@findMin nums.min()!!
        }
        val mid = (start + end) / 2 // always legit
        return if (nums[mid] > nums[start]) {
            findMin(nums, mid + 1, end)
        } else if (nums[start] < nums[mid]) {
            findMin(nums, start, mid + 1)
        } else if (nums[mid] > nums[end - 1]) { // start == mid
            findMin(nums, mid + 1, end)
        } else { // start == mid == end
            findMinInEqual(nums, start, end)
        }
    }

    private fun findMinInEqual(nums: IntArray, start: Int, end: Int): Int {
        var pos1 = start
        while (pos1 < end && nums[pos1] == nums[start]) {
            pos1++
        }
        var pos2 = end
        while (pos1 < pos2 && nums[pos2 - 1] == nums[end - 1]) {
            pos2--
        }
        return if (pos1 == pos2) {
            nums[pos1]
        } else {
            findMin(nums, pos1, min(pos2 + 1, end))
        }
    }
}

// @lc code=end

