#
# @lc app=leetcode.cn id=111 lang=python3
#
# [111] 二叉树的最小深度
#
from typing import List
from utils import *
# @lc code=start
from queue import Queue


class Solution:
    def minDepth(self, root: TreeNode) -> int:
        q = Queue()
        if root is None:
            return 0
        depth = 1
        q.put(root)
        while True:
            n = q.qsize()
            for i in range(n):
                node: TreeNode = q.get()
                if node.left is None and node.right is None:
                    return depth
                if node.left:
                    q.put(node.left)
                if node.right:
                    q.put(node.right)
            depth += 1


# @lc code=end
if __name__ == "__main__":
    f = Solution().minDepth
    assert f(Codec().deserialize('[3,9,20,null,null,15,7]')) == 2
    assert f(Codec().deserialize('[1]')) == 1
    assert f(Codec().deserialize('[]')) == 0
