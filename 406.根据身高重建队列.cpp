/*
 * @lc app=leetcode.cn id=406 lang=cpp
 *
 * [406] 根据身高重建队列
 */
#include <algorithm>
#include <iostream>
#include <vector>
using std::cout, std::endl;
using std::vector;

// @lc code=start
#include <cassert>
class Solution {
    int lowbit(int x) { return x & (-x); }

    int highbit(int x) {
        int ret = 1;
        while (x >>= 1) {
            ret <<= 1;
        }
        return ret;
    }

   public:
    vector<vector<int>> reconstructQueue(vector<vector<int>>& people) {
        int n = people.size();
        // sort by h (desc) and k
        std::sort(people.begin(), people.end(),
                  [](const vector<int>& p1, const vector<int>& p2) {
                      if (p1[0] == p2[0]) {
                          return p1[1] < p2[1];
                      }
                      return p1[0] > p2[0];
                  });
        // 树状数组
        vector<int> BIT = vector<int>(n + 1);
        for (int i = 1; i <= n; i++) {
            BIT[i] = lowbit(i);
        }

        // reversely insert every h into k th pos
        vector<vector<int>> result(n);
        int highbit_n = highbit(n);
        for (int i = n - 1; i >= 0; i--) {
            int h = people[i][0];
            int target_sum = people[i][1] + 1;
            // find pos to insert
            int cur_sum = 0, cur_pos = 0, incr = highbit_n;
            while (incr) {
                int try_pos = cur_pos + incr;
                if (try_pos <= n) {
                    if (cur_sum + BIT[try_pos] < target_sum) {
                        cur_sum += BIT[try_pos];
                        cur_pos = try_pos;
                    } else {
                        // cur_sum -= BIT[try_pos];
                    }
                }
                incr /= 2;
            }
            // offset is 1!
            int pos = cur_pos + 1;
            // clear pos (mark as inserted)
            for (int j = pos; j <= n; j += lowbit(j)) {
                BIT[j] -= 1;
            }
            // write to result
            result[pos - 1] = people[i];
        }

        return result;
    }
};
// @lc code=end

int main() {
    Solution s;

    std::vector<std::vector<int>> p;
    p.push_back({7, 0});
    p.push_back({4, 4});
    p.push_back({7, 1});
    p.push_back({5, 0});
    p.push_back({6, 1});
    p.push_back({5, 2});
    s.reconstructQueue(p);
    for (auto i : p) {
        std::cout << i[0] << ", " << i[1] << std::endl;
    }
}
