#
# @lc app=leetcode.cn id=117 lang=python3
#
# [117] 填充每个节点的下一个右侧节点指针 II
#
from typing import List
from utils import *


class Node:
    def __init__(self, val: int = 0, left: 'Node' = None, right: 'Node' = None, next: 'Node' = None):
        self.val = val
        self.left = left
        self.right = right
        self.next = next

# @lc code=start


class Solution:
    def connect(self, root: 'Node') -> 'Node':
        next_hyper = Node()

        node = root
        next_level = next_hyper

        while node:
            # link
            if node.left:
                next_level.next = node.left
                next_level = next_level.next
            # link
            if node.right:
                next_level.next = node.right
                next_level = next_level.next

            node = node.next
            if node is None:
                node = next_hyper.next
                next_hyper = Node()
                next_level = next_hyper

        return root


# @lc code=end
if __name__ == '__main__':
    nodes = [Node(i) for i in range(8)]
    nodes[1].left = nodes[2]
    nodes[1].right = nodes[3]
    nodes[2].left = nodes[4]
    nodes[2].right = nodes[5]
    nodes[3].right = nodes[7]

    Solution().connect(nodes[1])
    assert nodes[1].next is None

    assert nodes[2].next == nodes[3]
    assert nodes[3].next is None

    assert nodes[4].next == nodes[5]
    assert nodes[5].next == nodes[7]
    assert nodes[7].next is None
