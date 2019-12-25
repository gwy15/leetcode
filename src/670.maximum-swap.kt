/*
 * @lc app=leetcode id=670 lang=kotlin
 *
 * [670] Maximum Swap
 */

// @lc code=start
class Solution {
    fun maximumSwap(num: Int): Int {
        val numString: String = num.toString()
        val length = numString.length
        val lastOccurPos = IntArray(10, { i -> -1 })
        for ((index, char) in numString.withIndex()) {
            lastOccurPos[char.toString().toInt()] = index
        }
        for ((index, char) in numString.withIndex()) {
            val charInt = char.toString().toInt()
            for (nextNum in 9 downTo charInt+1) {
                if (lastOccurPos[nextNum] > index) {
                    var ans: Int = num
                    ans += (nextNum - charInt) * (10.0).pow(length - index - 1).toInt()
                    ans += (charInt - nextNum) * (10.0).pow(length - lastOccurPos[nextNum] - 1).toInt()
                    return ans
                }
            }
        }
        return num
    }
}
// @lc code=end

