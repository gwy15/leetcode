/*
 * @lc app=leetcode.cn id=976 lang=cpp
 *
 * [976] 三角形的最大周长
 */
#include <algorithm>
#include <vector>

// @lc code=start
class Solution {
public:
    int largestPerimeter(vector<int>& A) {
        if (A.size() < 3)
            return 0;
        std::make_heap(A.begin(), A.end());
        auto pop = [&A]() -> int {
            int a = A.front();
            std::pop_heap(A.begin(), A.end());
            A.pop_back();
            return a;
        };
        std::vector<int> abc;
        auto ok = [&abc]() -> bool {
            if (abc.size() < 3)
                return false;
            return abc[0] < abc[1] + abc[2];
        };
        // init
        abc.push_back(pop());
        abc.push_back(pop());
        abc.push_back(pop());
        while (!ok() && A.size()) {
            abc[0] = abc[1];
            abc[1] = abc[2];
            abc[2] = pop();
        }
        if (ok()) {
            return abc[0] + abc[1] + abc[2];
        } else {
            return 0;
        }
    }
};
// @lc code=end

