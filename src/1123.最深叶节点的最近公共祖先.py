#
# @lc app=leetcode.cn id=1123 lang=python3
#
# [1123] 最深叶节点的最近公共祖先
#
from prelude import *
# @lc code=start


class Solution:
    def lcaDeepestLeaves(self, root: TreeNode) -> TreeNode:

        def helper(root: TreeNode) -> (TreeNode, int):
            '返回当前树的最深叶节点、当前树最深叶节点高度（当前树高度）'
            if root is None:
                return (None, 0)
            left_ans, left_h = helper(root.left)
            right_ans, right_h = helper(root.right)
            # 如果左右高度相等，那就不用看其他的了，俺就是最近公共祖先
            if left_h == right_h:
                return (root, left_h + 1)
            if left_h > right_h:
                # 左边的中
                return (left_ans, left_h + 1)
            else:
                return (right_ans, right_h + 1)

        return helper(root)[0]


# @lc code=end
if __name__ == '__main__':
    def test(input, expected):
        calc = Solution().lcaDeepestLeaves(Codec().deserialize(input))
        if calc.val != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test('[3,5,1,6,2,0,8,null,null,7,4]', 2)
    test('[1]', 1)
    test('[0,1,3,null,2]', 2)
