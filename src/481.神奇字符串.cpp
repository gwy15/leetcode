/*
 * @lc app=leetcode.cn id=481 lang=cpp
 *
 * [481] 神奇字符串
 */

#include <iostream>
#include <queue>

// @lc code=start

const bool ONE = false;
const bool TWO = true;

class Solution {
   public:
    int magicalString(int n) {
        if (n < 6) {
            const int cache[] = {0, 1, 1, 1, 2, 3, 3};
            return cache[n];
        }

        std::queue<bool> ai;
        // [1, 2,] 2,      1, 1, 2, 1, 2, 2, 1221, ...
        //  |  | \   \\
        // [1, 2, 2,] 1, 1,      2, 1, 2, 2,
        ai.push(TWO);
        std::size_t length = 3, one_count = 1;
        bool group_is_one = true;
        // iter
        while (length < n) {
            bool group_length_is_two = ai.front();  // ONE or TWO
            ai.pop();
            int group_length = group_length_is_two + 1;
            // std::cout << 1 + a << " ";
            length += group_length;  // ONE -> 1, TWO -> 2
            // grow
            bool next_group = group_is_one ? ONE : TWO;
            ai.push(next_group);
            if (group_length_is_two) {  // push two items
                ai.push(next_group);
            }
            // sum one groups
            if (group_is_one) {             // if one only for odd i
                one_count += group_length;  // ONE -> 1, TWO -> 2
            }
            // flip
            group_is_one = !group_is_one;
        }
        // std::cout << std::endl;
        if ((length == n + 1) && !group_is_one) {
            return one_count - 1;
        }
        return one_count;
    }
};
// @lc code=end

int main() {
    // 12211 21221 22112 1122
    // 1  23 34 45  5677 89
    // int cases[][2] = {{6, 3},  {7, 4},  {9, 4},  {10, 5},
    //                   {12, 5}, {13, 6}, {14, 7}, {15, 7}};
    // for (auto ab : cases) {
    //     std::cout << "length = " << ab[0]
    //               << ", result = " << Solution().magicalString(ab[0])
    //               << ", expect " << ab[1] << std::endl;
    // }
    for (int i=0; i<100; i++) {
        std::cout << Solution().magicalString(i) << ", ";
    }
    std::cout << std::endl;
}
