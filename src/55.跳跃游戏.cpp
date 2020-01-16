/*
 * @lc app=leetcode.cn id=55 lang=cpp
 *
 * [55] 跳跃游戏
 */

// @lc code=start
class Solution {
public:
    bool canJump(vector<int>& nums) {
        const int N = nums.size();
        int max = 0;
        for (int i=0; i<N && i<=max; i++) {
            max = ::max(max, i + nums[i]);
            if (max >= (N-1))
                return true;
        }
        return false;
    }
};
// @lc code=end

