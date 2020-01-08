/*
 * @lc app=leetcode id=1238 lang=cpp
 *
 * [1238] circular-permutation-in-binary-representation
 */

#include <vector>
using namespace std;

// @lc code=start
class Solution {
public:
    vector<int> circularPermutation(int n, int start) {
        vector<int> res(1 << n);
        int MASK = (1 << n) - 1;
        // gray to binary
        int bin = grayToBin(start);
        for (int i=0; i <= MASK; i++) {
            res[i] = bin ^ (bin >> 1); // binary to gray
            bin = (bin + 1) & MASK;
        }
        return res;
    }

    static inline int grayToBin(int gray) {
        int b = gray;
        while (gray >>= 1) {
            b ^= gray;
        }
        // int b = 0;
        // for (int i = 30; i>=0; i--) {
        //     b &= ~(1 << i); // clear b[i]
        //     int bit = 1 & ((b >> (i+1)) ^ (gray >> i)); // b[i+1] ^ g[i]
        //     b |= (bit << i); // set b[i] = b[i+1] ^ g[i];
        // }
        return b;
    }
};
// @lc code=end

#include <iostream>
int main() {
    std::cout << Solution().grayToBin(4) << std::endl;
}
