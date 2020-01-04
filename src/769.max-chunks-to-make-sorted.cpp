/*
 * @lc app=leetcode id=769 lang=cpp
 *
 * [769] Max Chunks To Make Sorted
 */

#include <vector>
#include <algorithm>
using namespace std;

// @lc code=start
inline int max(int x, int y) {
    return (x > y) ? x : y;
}
class Solution {
public:
    int maxChunksToSorted(vector<int>& arr) {
        int blocks = 0;
        int max = -1;
        for (int i=0; i<arr.size(); i++) {
            max = ::max(max, arr[i]);
            if (max == i) {
                blocks ++;
            }
        }
        return blocks;
    }
};
// @lc code=end

