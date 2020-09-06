#
# @lc app=leetcode.cn id=107 lang=python3
#
# [107] 二叉树的层次遍历 II
#
from typing import List
from utils import *
# @lc code=start
from queue import Queue


class Solution:
    def levelOrderBottom(self, root: TreeNode) -> List[List[int]]:
        ans = []
        q = Queue()
        if root:
            q.put(root)
        while q.qsize():
            n = q.qsize()
            level = []
            for i in range(n):
                node = q.get()
                level.append(node.val)
                if node.left:
                    q.put(node.left)
                if node.right:
                    q.put(node.right)
            ans.append(level)
        return ans[::-1]


# @lc code=end
if __name__ == '__main__':
    def test(input, expected):
        calc = Solution().levelOrderBottom(Codec().deserialize(input))
        if calc != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test(
        '[3,9,20,null,null,15,7]',
        [[15, 7], [9, 20], [3]]
    )
    test(
        '[]',
        []
    )
