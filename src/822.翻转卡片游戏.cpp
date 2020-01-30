/*
 * @lc app=leetcode.cn id=822 lang=cpp
 *
 * [822] 翻转卡片游戏
 */
#include <vector>
#include <unordered_set>

// @lc code=start
class Solution {
public:
    int flipgame(vector<int>& fronts, vector<int>& backs) {
        std::size_t N = fronts.size();
        std::unordered_set<int> impossible;
        impossible.reserve(N); // max N impossibles

        for (int i=0; i<N; i++) {
            if (fronts[i] == backs[i]) {
                impossible.insert(fronts[i]);
            }
        }
        const int INF = 0x7fffffff;
        int result = INF;
        for (int num: fronts) {
            if (num < result && impossible.count(num) == 0) {
                result = num;
            }
        }
        for (int num: backs) {
            if (num < result && impossible.count(num) == 0) {
                result = num;
            }
        }
        return result == INF ? 0 : result;
    }
};
// @lc code=end

