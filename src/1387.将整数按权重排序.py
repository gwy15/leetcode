#
# @lc app=leetcode.cn id=1387 lang=python3
#
# [1387] 将整数按权重排序
#

# @lc code=start


class Solution:
    def getKth(self, lo: int, hi: int, k: int) -> int:
        steps = [-1] * (hi + 1)

        def find(x):
            if x == 1:
                return 0
            # cached
            if x <= hi and steps[x] != -1:
                return steps[x]

            ans = 0
            if x % 2 == 0:
                ans = find(x // 2) + 1
            else:
                ans = find(3 * x + 1) + 1
            # cache
            if x <= hi:
                steps[x] = ans
            return ans

        for i in range(1, hi + 1):
            steps[i] = find(i)

        ans = list(range(lo, hi+1))
        ans.sort(key=lambda x: (steps[x], x))
        return ans[k - 1]


# @lc code=end
if __name__ == "__main__":
    s = Solution()
    assert s.getKth(12, 15, 2) == 13
    assert s.getKth(1, 1, 1) == 1
    assert s.getKth(7, 11, 4) == 7
    assert s.getKth(10, 20, 5) == 13
    assert s.getKth(1, 1000, 777) == 570
