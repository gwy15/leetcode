#
# @lc app=leetcode.cn id=889 lang=python3
#
# [889] 根据前序和后序遍历构造二叉树
#
from prelude import *
# @lc code=start
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right


class Solution:
    def constructFromPrePost(self, pre: List[int], post: List[int]) -> TreeNode:
        # print(pre, post)
        n = len(pre)
        # assert len(pre) == len(post)
        if len(pre) == 0:
            return None
        if pre[0] != post[-1]:
            return None
        # 找左树和右树
        if len(pre) == 1:
            return TreeNode(pre[0])
        left = pre[1]
        right = post[-2]
        # i 把数组分割为   0 <= i <= n-1
        #  pre[1, i],  pre[i+1, n)
        # post[0, i), post[i, n-1)
        # 需要有 left = post[i-1]
        #      right = pre[i+1]
        for i in range(n):
            #   左树
            if (i == 0 or left == post[i-1]) and (i+1 == n or pre[i+1] == right):
                # print(pre, post, i, pre[i])
                left_tree = self.constructFromPrePost(pre[1:i+1], post[:i])
                # 没找到，不合法
                if left_tree is None and i > 0:
                    continue
                right_tree = self.constructFromPrePost(pre[i+1:n], post[i:n-1])
                if right_tree is None and i+1 < n:
                    continue
                # ok
                node = TreeNode(pre[0])
                node.left = left_tree
                node.right = right_tree
                return node
        return None


# @lc code=end
if __name__ == '__main__':
    def test(pre, post, expected):
        calc = Solution().constructFromPrePost(pre, post)
        calc = Codec().serialize(calc)
        if calc != expected:
            print(f'case failed: `{pre, post}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test(
        pre=[1, 2, 4, 5, 3, 6, 7], post=[4, 5, 2, 6, 7, 3, 1],
        expected='[1,2,3,4,5,6,7]'
    )
