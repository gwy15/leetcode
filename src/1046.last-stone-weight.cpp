/*
 * @lc app=leetcode id=1046 lang=cpp
 *
 * [1046] Last Stone Weight
 */

#include <vector>
#include <iostream>
#include <algorithm>
using namespace std;

// @lc code=start
class Solution {
public:
    int lastStoneWeight(vector<int>& stones) {
        std::make_heap(stones.begin(), stones.end());
        while (true) {
            if (stones.size() <= 1) {
                if (stones.size() == 0) {
                    return 0;
                }
                return stones[0];
            }
            // stones.size >= 2!
            // now fetch two stones
            std::pop_heap(stones.begin(), stones.end());
            auto s1 = stones.back();
            stones.pop_back();

            std::pop_heap(stones.begin(), stones.end());
            auto s2 = stones.back();
            stones.pop_back();
            
            if (s1 != s2) {
                stones.push_back(std::abs(s1 - s2));
                std::push_heap(stones.begin(), stones.end());
            }
        }
    }
};
// @lc code=end

int main() {
    auto s = std::vector<int> { 2,7,4,1,8,1 };
    Solution().lastStoneWeight(s);
}