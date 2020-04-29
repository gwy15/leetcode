/*
 * @lc app=leetcode.cn id=406 lang=cpp
 *
 * [406] 根据身高重建队列
 */
#include <algorithm>
#include <iostream>
#include <vector>
using std::cout;
using std::endl;
using std::vector;

// @lc code=start
class Solution {
    int lowbit(int x) { return x & (-x); }

    int highbit(int x) {
        int ret = 1;
        while (x >>= 1) {
            ret <<= 1;
        }
        return ret;
    }

    int binary_search_bit_for_sum(const vector<int>& BIT, int target_sum,
                                  int init_incr) {
        int n = BIT.size() - 1;
        int cur_sum = 0, cur_pos = 0, incr = init_incr;
        while (incr) {
            int try_pos = cur_pos + incr;
            if (try_pos <= n) {
                if (cur_sum + BIT[try_pos] < target_sum) {
                    cur_sum += BIT[try_pos];
                    cur_pos = try_pos;
                }
            }
            incr /= 2;
        }
        // +1 because cur_sum < target_sum. +1 makes equal
        return cur_pos + 1;
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
        // init BIT
        vector<int> BIT = vector<int>(n + 1);
        for (int i = 1; i <= n; i++) {
            BIT[i] = lowbit(i);
        }

        // reversely insert every people into k-th pos
        vector<vector<int>> result(n);
        // cache a log n operation
        const int highbit_n = highbit(n);
        for (int i = n - 1; i >= 0; i--) {
            const int target_sum = people[i][1] + 1;
            // find pos to insert
            int pos = binary_search_bit_for_sum(BIT, target_sum, highbit_n);
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
