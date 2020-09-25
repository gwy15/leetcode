#
# @lc app=leetcode.cn id=106 lang=python3
#
# [106] 从中序与后序遍历序列构造二叉树
#
from typing import List
from utils import *
# @lc code=start


class Solution:
    def buildTree(self, inorder: List[int], postorder: List[int]) -> TreeNode:
        n = len(inorder)
        inorder_map = {num: i for (i, num) in enumerate(inorder)}

        def helper(inorder_range, postorder_range):
            if inorder_range[0] == inorder_range[1]:
                return None
            root_val = postorder[postorder_range[1] - 1]
            tree = TreeNode(root_val)
            # 根的位置
            inorder_root_index = inorder_map[root_val]
            # 左树的长度
            left_tree_length = inorder_root_index - inorder_range[0]

            left_tree = helper(
                [inorder_range[0], inorder_root_index],
                [postorder_range[0], postorder_range[0] + left_tree_length]
            )
            right_tree = helper(
                [inorder_root_index+1, inorder_range[1]],
                [postorder_range[0] + left_tree_length, postorder_range[1] - 1]
            )
            tree.left = left_tree
            tree.right = right_tree

            return tree

        return helper([0, n], [0, n])


# @lc code=end
if __name__ == '__main__':
    def test(input, expected):
        inorder, postorder = input
        calc = Solution().buildTree(inorder, postorder)
        expected = Codec().deserialize(expected)
        if calc != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test(
        [
            [9, 3, 15, 20, 7],
            [9, 15, 7, 20, 3]
        ],
        '[3,9,20,null,null,15,7]'
    )
