/*
 * @lc app=leetcode.cn id=1269 lang=cpp
 *
 * [1269] 停在原地的方案数
 */
#include <cstdint>
#include <cstring>
#include <iostream>
#include <vector>
using std::uint32_t;
using std::vector;

// @lc code=start
class Solution {
    const static constexpr uint32_t MOD = 1000000000 + 7;

   public:
    int numWays(int steps, int arrLen) {
        if (arrLen == 1) return 1;
        if (arrLen > steps) {
            arrLen = steps;
        }

        using VecT = vector<uint32_t>;
        VecT options(arrLen, 0);
        options[0] = 1;

        VecT tmp;
        for (int i = 0; i < steps; i++) {
            // reset
            tmp = VecT(arrLen, 0);
            // start
            tmp[0] = (options[0] + options[1]) % MOD;
            // end
            tmp[arrLen - 1] = (options[arrLen - 2] + options[arrLen - 1]) % MOD;

            for (int j = 1; j < arrLen - 1; j++) {
                tmp[j] = (options[j - 1] + options[j] + options[j + 1]) % MOD;
            }
            options = std::move(tmp);
        }
        return options[0];
    }
};
// @lc code=end

int main() {
    std::cout << Solution().numWays(3, 2) << std::endl;
    std::cout << Solution().numWays(2, 4) << std::endl;
}
