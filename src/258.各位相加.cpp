/*
 * @lc app=leetcode.cn id=258 lang=cpp
 *
 * [258] 各位相加
 */

// @lc code=start
class Solution {
public:
    int addDigits(int num) {
        while (true) {
            int sum = 0;
            while (num) {
                sum += num % 10;
                num /= 10;
            }
            if (sum < 10)
                return sum;
            num = sum;
        }
    }
};
// @lc code=end

