/*
 * @lc app=leetcode id=89 lang=cpp
 *
 * [89] Gray Code
 */

#include <vector>
using namespace std;

// @lc code=start
class Solution {
public:
    vector<int> grayCode(int n) {
        vector<int> res(1 << n);
        for (int b = 0; b < (1<<n); b++) {
            res[b] = toGray(b);
        }
        return res;
    }
    static inline int toGray(int b) {
        return b ^ (b >> 1);
    }
};
// @lc code=end

