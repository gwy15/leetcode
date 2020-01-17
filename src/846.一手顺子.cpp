/*
 * @lc app=leetcode.cn id=846 lang=cpp
 *
 * [846] 一手顺子
 */
#include <vector>
#include <algorithm>
#include <map>
#include <iostream>

// @lc code=start
class Solution {
   public:
    bool isNStraightHand(std::vector<int>& hand, int W) {
        if (hand.size() % W != 0) return false;
        // 贪心，最小的数肯定是顺子的最小开始，去除连续 W 个就可以了。
        std::map<int, int> cards;
        for (auto card: hand) {
            auto iter = cards.find(card);
            if (iter != cards.end()) {
                iter->second++;
            } else {
                cards.insert(std::pair<int, int>(card, 1));
            }
        }
        // erase
        while (cards.size()) {
            int base = cards.begin() -> first; // start from base
            for (int i=0; i<W; i++) {
                int card = base + i;
                auto iter = cards.find(card);
                if (iter == cards.end()) { // not found
                    return false;
                }
                (iter->second)--; // erase once
                if (iter->second == 0) {
                    cards.erase(iter);
                }
            }
        }
        return true;
    }
};
// @lc code=end

int main() {
    std::vector<int> hand {1,2,3,6,2,3,4,7,8};
    std::cout << Solution().isNStraightHand(
        hand, 3
    ) << std::endl;
}
