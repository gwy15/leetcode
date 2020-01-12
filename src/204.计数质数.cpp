/*
 * @lc app=leetcode.cn id=204 lang=cpp
 *
 * [204] 计数质数
 */

// @lc code=start
class Solution {
public:
    int countPrimes(int N) {
        int pNum = 0;
        vector<bool> flag(N + 1, true);
        vector<int> p;

        for (int i = 2; i < N; i++) {
            if (flag[i]) {
                p.push_back(i);
            }

            for (int j = 0; j < p.size() && p[j] * i < N; j++) {
                flag[p[j] * i] = false;
                if (i % p[j] == 0) break;
            }
        }
        return p.size();
    }
};
// @lc code=end

