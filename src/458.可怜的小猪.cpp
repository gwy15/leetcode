/*
 * @lc app=leetcode.cn id=458 lang=cpp
 *
 * [458] 可怜的小猪
 */

// @lc code=start
class Solution {
public:
    int poorPigs(int buckets, int minutesToDie, int minutesToTest) {
        int rounds = (minutesToTest / minutesToDie);
        int t = rounds + 1;
        return ::ceil(::log(buckets) / ::log(t));
    }
};
// @lc code=end

