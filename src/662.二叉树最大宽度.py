#
# @lc app=leetcode.cn id=662 lang=python3
#
# [662] 二叉树最大宽度
#
from typing import List
from utils import *
# @lc code=start
from queue import Queue


class Solution:
    def widthOfBinaryTree(self, root: TreeNode) -> int:
        INF = float('inf')
        q = Queue()
        ans = 0
        q.put((root, 0))
        while q.qsize():
            n = q.qsize()
            left, right = INF, -INF
            for _ in range(n):
                (node, pos) = q.get()
                left = min(left, pos)
                right = max(right, pos)
                if node.left:
                    q.put((node.left, 2 * pos))
                if node.right:
                    q.put((node.right, 2 * pos + 1))
            ans = max(ans, right - left + 1)
        return ans


# @lc code=end
if __name__ == '__main__':
    def test(input, expected):
        calc = Solution().widthOfBinaryTree(Codec().deserialize(input))
        if calc != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test('[1,3,null,5,3]', 2)
    test('[1]', 1)
    test('[1,3,2,5]', 2)
    test('[1,3,2,5,null,null,9,6,null,null,7]', 8)
    t = '[0,0,0,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null,null,0,0,null]'
    # print(Codec().deserialize(t))
    test(t, 2)
