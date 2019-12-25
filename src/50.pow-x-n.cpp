/*
 * @lc app=leetcode id=50 lang=cpp
 *
 * [50] Pow(x, n)
 */
class Solution {
   public:
    double myPow(double x, int _n) {
        int64_t n = _n;
        double res = 1.0;
        if (n < 0) {
            n = -n;
            x = 1.0 / x;
        }
        while (n) {
            if (n & 1) {
                res *= x;
            }
            x *= x;
            n >>= 1;
        }
        return res;
    }
};
