/*
 * @lc app=leetcode.cn id=1252 lang=cpp
 *
 * [1252] 奇数值单元格的数目
 */

#include <vector>
using std::vector;

// @lc code=start
class Solution {
public:
    int oddCells(int n, int m, vector<vector<int>>& indices) {
        vector<int> x_count(n, 0), y_count(m, 0);
        for (const auto& p: indices) {
            int x = p[0], y = p[1];
            x_count[x]++;
            y_count[y]++;
        }
        // sum y
        int y_odd_count = 0;
        for (auto y: y_count) {
            if (y & 1) y_odd_count++;
        }
        int y_even_count = m - y_odd_count;

        int sum = 0;
        for (auto x: x_count) {
            if (x & 1) {
                sum += y_even_count;
            } else {
                sum += y_odd_count;
            }
        }
        return sum;
    }
};
// @lc code=end

