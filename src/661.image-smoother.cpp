/*
 * @lc app=leetcode id=661 lang=cpp
 *
 * [661] Image Smoother
 */
#include <vector>
using std::size_t;
using std::vector;
#include <iostream>
// @lc code=start

template <int di, int dj, class T>
inline void _do_sum(T& M, size_t i, size_t j) {
    M[i][j] += ((M[i + di][j + dj] & 0xFF) << 16);
}
// Helper class to do iterations
template <int di, int dj, int fromDeltaI, int fromDeltaJ, int toDeltaI, int toDeltaJ, class T>
struct _run_loop_helper
{
    static void inline run(T& M, size_t i, size_t j) {
        _do_sum<di, dj, T>(M, i, j);
        _run_loop_helper<di + 1, dj, fromDeltaI, fromDeltaJ, toDeltaI, toDeltaJ, T>::run(M, i, j);
    }
};

template <int dj, int fromDeltaI, int fromDeltaJ, int toDeltaI, int toDeltaJ, class T>
struct _run_loop_helper<toDeltaI, dj, fromDeltaI, fromDeltaJ, toDeltaI, toDeltaJ, T>
{
    static void inline run(T& M, size_t i, size_t j) {
        _do_sum<toDeltaI, dj, T>(M, i, j);
        _run_loop_helper<fromDeltaI, dj+1, fromDeltaI, fromDeltaJ, toDeltaI, toDeltaJ, T>::run(M, i, j);
    }
};

template <int fromDeltaI, int fromDeltaJ, int toDeltaI, int toDeltaJ, class T>
struct _run_loop_helper<toDeltaI, toDeltaJ, fromDeltaI, fromDeltaJ, toDeltaI, toDeltaJ, T>
{
    static void inline run(T& M, size_t i, size_t j) {
        _do_sum<toDeltaI, toDeltaJ, T>(M, i, j);
    }
};

template <int fromDeltaI, int fromDeltaJ, int toDeltaI, int toDeltaJ, class T>
inline void sumRange(T& M, size_t i, size_t j) {
    _run_loop_helper<fromDeltaI, fromDeltaJ, fromDeltaI, fromDeltaJ, toDeltaI, toDeltaJ, T>::run(M, i, j);
}


// 
class Solution {
   public:
    vector<vector<int>>& imageSmoother(vector<vector<int>>& M) {
        if (M.empty()) return M;
        const size_t X = M.size();
        const size_t Y = M[0].size();

        if (X == 1 && Y == 1) return M;
        // === sum
        if (Y == 1) { // horizontal
            sumRange< 0, 0, 1, 0>(M,   0, 0);
            sumRange<-1, 0, 0, 0>(M, X-1, 0);
            for (size_t i=1; i<(X-1); i++)
                sumRange<-1, 0, 1, 0>(M, i, 0);
            M[0][0] >>= 17; // 16 + 1
            M[X-1][0] >>= 17;
            for (size_t i=1; i<(X-1); i++)
                M[i][0] /= (3 << 16);
            return M;
        }
        if (X == 1) { // vertical
            sumRange<0,  0, 0, 1>(M, 0,   0);
            sumRange<0, -1, 0, 0>(M, 0, Y-1);
            for (size_t j=1; j<(Y-1); j++)
                sumRange<0, -1, 0, 1>(M, 0, j);
            M[0][0] >>= 17; // 16 + 1
            M[0][Y-1] >>= 17;
            for (size_t j=1; j<(Y-1); j++)
                M[0][j] /= (3 << 16);
            return M;
        }

        // corner
        sumRange< 0,  0,  1, 1>(M,   0,   0); // LU
        sumRange<-1,  0,  0, 1>(M, X-1,   0); // RU
        sumRange< 0, -1,  1, 0>(M,   0, Y-1); // LD
        sumRange<-1, -1,  0, 0>(M, X-1, Y-1); // RD
        // side
        for (size_t i=1; i<(X-1); i++) { // horizontal
            sumRange<-1,  0, 1, 1>(M, i,   0);
            sumRange<-1, -1, 1, 0>(M, i, Y-1);
        }
        for (size_t j=1; j<(Y-1); j++) { // vertical
            sumRange< 0, -1, 1, 1>(M,   0, j);
            sumRange<-1, -1, 0, 1>(M, X-1, j);
        }
        // center
        for (size_t i=1; i<(X-1); i++) {
            for (size_t j=1; j<(Y-1); j++) {
                sumRange<-1, -1, 1, 1>(M, i, j);
            }
        }

        // === divide
        // corner
        M[  0][  0] >>= 18; // 16 + 2(/4)
        M[X-1][  0] >>= 18;
        M[  0][Y-1] >>= 18;
        M[X-1][Y-1] >>= 18;
        // side
        const constexpr int D = (6 << 16); // >>= 16, / 6
        for (size_t i=1; i<(X-1); i++) { // horizontal
            M[i][0] /= D;
            M[i][Y-1] /= D;
        }
        for (size_t j=1; j<(Y-1); j++) { // vertical
            M[0][j] /= D;
            M[X-1][j] /= D;
        }
        // center
        const constexpr int DD = (9 << 16);
        for (size_t i=1; i<(X-1); i++) {
            for (size_t j=1; j<(Y-1); j++) {
                M[i][j] /= DD;
            }
        }

        return M;
    }
};
// @lc code=end


int main() {
    using T = vector<vector<int>>;
    T arr = {{1}};
    // vector<vector<int>> arr = {
    //     {1,1,1},
    //     {1,0,1},
    //     {1,1,1}
    // };
    Solution().imageSmoother( arr );
    for (auto r: arr) {
        for (auto x: r) {
            std::cout << x << ' ';
        }
        std::cout << std::endl;
    }
    std::cout << std::endl;
}
