/*
 * @lc app=leetcode.cn id=1175 lang=cpp
 *
 * [1175] 质数排列
 */
#include <vector>
using std::vector;

// @lc code=start
class Solution {
   public:
    int primeN(const int N) {
        int pNum = 0;
        vector<bool> flag(N + 1, true);
        vector<int> p;

        for (int i = 2; i <= N; i++) {
            if (flag[i]) {
                p.push_back(i);
            }

            for (int j = 0; j < p.size() && p[j] * i <= N; j++) {
                flag[p[j] * i] = false;
                if (i % p[j] == 0) break;
            }
        }
        return p.size();
    }

    int numPrimeArrangements(int n) {
        using ULL = unsigned long long;
        const ULL MOD = 1000*1000*1000 + 7;

        const int primes = primeN(n);
        const int others = n - primes;
        ULL res = 1;
        for (ULL i=2; i<=primes; i++) {
            res = (res * i) % MOD;
        }
        for (ULL i=2; i<=others; i++) {
            res = (res * i) % MOD;
        }
        return (int)res;
    }
};
// @lc code=end
