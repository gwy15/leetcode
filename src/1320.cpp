#include <algorithm>
#include <climits>
#include <cstdint>
#include <functional>
#include <iostream>
#include <string>
#include <vector>
using std::function;
using std::min;
using std::string;
using std::vector;

#define p(i, j, ii, jj) \
    printf("(%d,%d)@%d -> (%d,%d)@%d\n", i, j, dp[i][j], ii, jj, dp[ii][jj]);

class Solution {
   private:
    inline static int char2int(char c) { return c - 'A'; }
    inline static int abs(int x) { return x >= 0 ? x : -x; }
    inline static int x(int c) { return c % 6; }
    inline static int y(int c) { return c / 6; }
    inline static int distance(int c1, int c2) {
        return abs(x(c1) - x(c2)) + abs(y(c1) - y(c2));
    }

   public:
    int minimumDistance(string word) {
        function<int(int, int)> dist;
        dist = [&word, &dist](int i, int j) -> int {
            // i < j
            if (i > j) return dist(j, i);
            if (i == 0)
                return 0;
            else
                return distance(char2int(word[i - 1]), char2int(word[j - 1]));
        };
#ifndef p
#define p(i, j, x, y) ;
#endif

        // init dp array
        int N = word.length();
        // dp[i][j] := min dist for left@i, right@j
        vector<vector<int>> dp(N + 1, vector<int>(N + 1, INT_MAX));
        // init
        for (int j = 1; j <= N; j++) dp[0][j] = dp[j][0] = 0;

        for (int i = 1; i < N; i++) {
            // grow on bigger index
            auto dist_i_i1 = dist(i, i + 1);
            for (int j = 0; j < i; j++) {  // j < i
                dp[i + 1][j] = dp[i][j] + dist_i_i1;
                dp[j][i + 1] = dp[j][i] + dist_i_i1;
                p(i, j, i + 1, j);
            }
            // grow on smaller index
            for (int j = 0; j < i; j++) {  // i,j -> i,i+1
                int dist_j_i1 = dist(j, i + 1);
                dp[i + 1][i] = min(dp[i + 1][i], dp[j][i] + dist_j_i1);
                dp[i][i + 1] = min(dp[i][i + 1], dp[i][j] + dist_j_i1);
                p(i, j, i, i + 1);
            }
        }
        // get result
        int result = INT_MAX;
        for (int j = 0; j <= N; j++) {
            result = min(result, dp[N][j]);
            result = min(result, dp[j][N]);
        }
        return result;
    }
};

int main() {
    std::cout << Solution().minimumDistance("CAKE") << std::endl;
    return 0;
}
