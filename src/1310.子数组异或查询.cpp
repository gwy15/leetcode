/*
 * @lc app=leetcode.cn id=1310 lang=cpp
 *
 * [1310] 子数组异或查询
 */
#include <vector>
using std::vector;

// @lc code=start
class Solution {
public:
    vector<int> xorQueries(vector<int>& arr, vector<vector<int>>& queries) {
        vector<int> x(arr);
        for (auto i=1; i<x.size(); i++) {
            x[i] ^= x[i-1];
        }
        vector<int> result(queries.size());
        for (auto i=0; i<queries.size(); i++) {
            int left = queries[i][0];
            int right = queries[i][1];
            if (left == 0) {
                result[i] = x[right];
            } else {
                result[i] = result[i] = x[right] ^ x[left - 1];
            }
        }
        return result;
    }
};
// @lc code=end


