class Solution {
    fun mincostTickets(days: IntArray, costs: IntArray): Int {
        val length = days.size
        var dp = IntArray(length, {_->0})
        dp[0] = cost[1]
        for (i in 1 until length) {

        }
        return costs[1]
    }
}

fun main() {
    val price = Solution().mincostTickets(intArrayOf(1,4,6,7,8,20), intArrayOf(2,7,15));
    println(price)
}
