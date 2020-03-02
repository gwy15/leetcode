/*
 * @lc app=leetcode.cn id=837 lang=cpp
 *
 * [837] 新21点
 */

#include <iostream>
#include <vector>

// @lc code=start
class Solution {
   public:
    double new21Game(const int N, const int K, const int W) const {
        if (K == 0) {
            return K <= N ? 1.0 : 0.0;
        }
        // p[i] = prop of sum bypass i
        std::vector<double> p(K + W, 0.0);
        p[0] = 1.0;
        //
        const double one_over_w = 1.0 / W;
        //
        double sum_part = p[0];  // sum_part = p[i-W] + ... P[i-1]
        int sum_length = 1;
        for (int i = 1; i < K + W; i++) {
            // p[i] = 1/w * sum(p[i-W] + P[i-W+1] + ... + P[i-1])
            p[i] = one_over_w * (sum_part);
            if (i < K) {  // stop suming
                if (sum_length < W) {
                    sum_part += p[i];
                    sum_length++;
                } else {
                    sum_part += p[i] - p[i - W];
                }
            } else if (i - W >= 0) {
                sum_part -= p[i - W];
            }
            // std::cout << "p[" << i << "] = " << p[i] << std::endl;
        }
        //
        double sum = 0;
        const int upper_bound = std::min(K+W-1, N);
        for (int i = K; i <= upper_bound; i++) {
            sum += p[i];
        }
        return sum;
    }
};
// @lc code=end

int main() {
    std::cout << Solution().new21Game(10, 1, 10) << std::endl;
    std::cout << Solution().new21Game(6, 1, 10) << std::endl;
    std::cout << Solution().new21Game(21, 17, 10) << std::endl;
    std::cout << Solution().new21Game(1, 0, 1) << std::endl;
    std::cout << Solution().new21Game(1, 0, 2) << std::endl;
    std::cout << Solution().new21Game(98, 33, 68) << std::endl;
    std::cout << Solution().new21Game(9, 3, 7) << std::endl;
    std::cout << Solution().new21Game(4, 2, 3) << std::endl;
}
