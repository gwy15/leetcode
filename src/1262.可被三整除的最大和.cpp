/*
 * @lc app=leetcode.cn id=1262 lang=cpp
 *
 * [1262] 可被三整除的最大和
 */
#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;

// @lc code=start
class Solution {
   public:
    int maxSumDivThree(vector<int>& nums) {
        return maxSumDivThree_DP(nums);
        // return maxSumDivThree2_greedy(nums);
    }
    int maxSumDivThree_DP(vector<int>& nums) {
        // through DP
        // define dp[i][j] = max sum of [0, i) items
        // sum at j (mod 3)
        int dp[3] = {0, 0, 0};
        int dp2[3] = {0, 0, 0};
        for (int i = 0; i < nums.size(); i++) {
            int n = nums[i];

            // do dp
            for (int j = 0; j < 3; j++) {  // from j
                if (dp[j] || j == 0) {     // allow from dp[0]
                    dp2[(j + n) % 3] = max(dp[(j + n) % 3], dp[j] + n);
                }
            }
            for (int j = 0; j < 3; j++) dp[j] = dp2[j];
        }
        return dp[0];
    }

    int maxSumDivThree2_greedy(vector<int>& nums) {
        // through greedy
        const int _INT_MAX = 100000;
        int sizes_mod[3] = {0, 0, 0};
        int subSums[3] = {0};
        int min_mod_n[3][2] = {
            {_INT_MAX, _INT_MAX}, {_INT_MAX, _INT_MAX}, {_INT_MAX, _INT_MAX}};
        for (auto n : nums) {
            int mod = n % 3;
            sizes_mod[mod]++;             // count + 1
            subSums[mod] += n;            // sum
            if (n < min_mod_n[mod][1]) {  // mark two smallest
                if (n < min_mod_n[mod][0]) {
                    min_mod_n[mod][1] = min_mod_n[mod][0];
                    min_mod_n[mod][0] = n;
                } else {
                    min_mod_n[mod][1] = n;
                }
            }
        }

        int sum = 0;
        int totalSum = subSums[0] + subSums[1] + subSums[2];
        for (int i = 0; i < 3; i++) {
            for (int j = 0; j < 3; j++) {
                int tempSum = totalSum;
                for (int ii = 0; ii < i; ii++) tempSum -= min_mod_n[1][ii];
                for (int jj = 0; jj < j; jj++) tempSum -= min_mod_n[2][jj];
                if (tempSum % 3 == 0) sum = max(sum, tempSum);
            }
        }
        return sum;
    }
};
// @lc code=end

int main() {
    auto v = vector<int>({3, 6, 5, 1, 8});
    std::cout << Solution().maxSumDivThree(v) << std::endl;
}
