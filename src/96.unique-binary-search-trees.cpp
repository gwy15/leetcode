/*
 * @lc app=leetcode id=96 lang=cpp
 *
 * [96] Unique Binary Search Trees
 */

// @lc code=start
class Solution {
private:
    int* buff;
public:
    Solution() {
        buff = nullptr;
    }
    int numTrees(int n) {
        if (buff == nullptr) {
            buff = new int[n + 2]{0};
            buff[0] = 1;
        }
        if (buff[n] != 0) {
            return buff[n];
        }
        
        int sum = 0;
        for (int k=1; k<=n; k++) {
            sum += numTrees(k-1) * numTrees(n-k);
        }
        buff[n] = sum;
        return sum;
    }
    ~Solution() {
        delete[] buff;
    }
};
// @lc code=end

