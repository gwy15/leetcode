#
# @lc app=leetcode.cn id=96 lang=python3
#
# [96] 不同的二叉搜索树
#

# @lc code=start


class Solution:
    def numTrees_simple(self, n: int) -> int:
        f = [1] * (n + 1)
        for root in range(1, n+1):
            # f -> [1, root]
            f[root] = sum(
                # [1, i-1]  [i+1, root]
                f[i-1] * f[root-i]
                for i in range(1, root+1)
            )
        return f[n]

    def numTrees_math(self, n: int) -> int:
        result = 1
        for i in range(1, n+1):
            # result = result * 2 * i * (2 * i - 1) // (
            #     i * (i + 1)
            # )
            result = result * 2 * (2 * i - 1) // (i + 1)

        return result

    def numTrees(self, n):
        return self.numTrees_math(n)


# @lc code=end
if __name__ == "__main__":
    f = Solution().numTrees
    assert f(0) == 1
    assert f(1) == 1
    assert f(2) == 2
    assert f(3) == 5
    assert f(5) == 42
