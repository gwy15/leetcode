/*
 * @lc app=leetcode id=223 lang=cpp
 *
 * [223] Rectangle Area
 */

// @lc code=start
using LL = long long;
class Solution {
public:
    LL overlapLength(int left1, int right1, int left2, int right2) {
        int left = max(left1, left2);
        int right = min(right1, right2);
        return LL(right) - LL(left);
    }

    int computeArea(int A, int B, int C, int D, int E, int F, int G, int H) {
        LL a1 = (C-A) * (D-B);
        LL a2 = (G-E) * (H-F);
        LL area = a1 + a2;
        
        LL x = overlapLength(A, C, E, G);
        LL y = overlapLength(B, D, F, H);
        if (x > 0 && y > 0) {
            area -= x * y;
        }
        return area;
    }
};
// @lc code=end

