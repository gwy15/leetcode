#
# @lc app=leetcode.cn id=386 lang=python3
#
# [386] 字典序排数
#
from typing import List
# @lc code=start


class Solution:
    def lexicalOrder(self, n: int) -> List[int]:
        ans = []

        def list_2_num(digits):
            val = 0
            for digits in digits:
                val = val * 10 + digits
            return val

        def dfs(root):
            # push root
            val = list_2_num(root)
            if val > n:
                return
            ans.append(val)
            for item in range(10):
                root.append(item)
                dfs(root)
                root.pop()

        for i in range(1, 10):
            dfs([i])
        return ans


# @lc code=end
if __name__ == "__main__":
    f = Solution().lexicalOrder
    assert f(13) == [1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9]
    assert f(1) == [1]
