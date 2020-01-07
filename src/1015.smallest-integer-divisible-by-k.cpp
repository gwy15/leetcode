/*
 * @lc app=leetcode id=1015 lang=cpp
 *
 * [1015] Smallest Integer Divisible by K
 */
#include <iostream>
#include <vector>
using namespace std;

// @lc code=start
class Solution {
   public:
    int smallestRepunitDivByK(int K) {
        if (K % 2 == 0) {
            return -1;
        }
        vector<bool> flag(K, false);
        int n = 1 % K;
        int length = 1;
        while (true) {
            // std::cout << "n = " << n << ", length = " << length << std::endl;
            if (n == 0) {  // divided
                return length;
            } else if (flag[n]) {  // visited, loop
                return -1;
            }

            // std::cout << "flag[" << n << "] set." << std::endl;
            flag[n] = true;  // this ensures loop ends
            n = (10 * n + 1) % K;
            length++;
        }
        return -1;
    }
};
// @lc code=end

int main() {
    std::cout << Solution().smallestRepunitDivByK(3) << std::endl;
    return 0;
}
