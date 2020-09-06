#
# @lc app=leetcode.cn id=501 lang=python3
#
# [501] 二叉搜索树中的众数
#
from typing import List
from utils import *
# @lc code=start


class Solution:
    def findMode(self, root: TreeNode) -> List[int]:
        max_freq, ans = 0, []
        cur_freq, cur_val = 0, -(1 << 30)

        def visit(node: TreeNode):
            nonlocal max_freq, ans, cur_freq, cur_val
            if node is None:
                return
            visit(node.left)
            #
            # count
            if node.val == cur_val:
                cur_freq += 1
            else:
                cur_freq, cur_val = 1, node.val
            # max
            if cur_freq == max_freq:
                ans.append(cur_val)
            elif cur_freq > max_freq:
                max_freq = cur_freq
                if not(len(ans) == 1 and ans[0] == cur_val):
                    ans = [cur_val]

            visit(node.right)

        visit(root)

        return ans


# @lc code=end
if __name__ == '__main__':
    def test(input, expected):
        calc = Solution().findMode(Codec().deserialize(input))
        if calc != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test('[1,null,2,2]',        [2])
    test('[1,1,2,null,null,2]', [1, 2])
    test('[]', [])
