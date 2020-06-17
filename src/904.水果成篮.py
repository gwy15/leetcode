#
# @lc app=leetcode.cn id=904 lang=python3
#
# [904] 水果成篮
#
from typing import List
# @lc code=start


class Solution:
    def totalFruit(self, tree: List[int]) -> int:
        n = len(tree)
        # [i, j)
        i, j = 0, 0
        counter = {}
        ans = 0
        while j < n:
            # move right until illegal
            counter[tree[j]] = counter.get(tree[j], 0) + 1
            j += 1

            # move left
            while len(counter) == 3:
                if counter[tree[i]] == 1:
                    counter.pop(tree[i])
                else:
                    counter[tree[i]] -= 1
                i += 1
            ans = max(ans, j - i)
        return ans


# @lc code=end
if __name__ == "__main__":
    s = Solution()

    def test(tree, ans):
        assert s.totalFruit(tree) == ans
    test([1, 2, 1], 3)
    test([0, 1, 2, 2], 3)
    test([1, 2, 3, 2, 2], 4)
    test([3, 3, 3, 1, 2, 1, 1, 2, 3, 3, 4], 5)
