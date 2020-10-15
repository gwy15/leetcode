#
# @lc app=leetcode.cn id=116 lang=python3
#
# [116] 填充每个节点的下一个右侧节点指针
#

# Definition for a Node.
class Node:
    def __init__(self, val: int = 0, left: 'Node' = None, right: 'Node' = None, next: 'Node' = None):
        self.val = val
        self.left = left
        self.right = right
        self.next = next
# @lc code=start


class Solution:
    def connect(self, root: 'Node') -> 'Node':
        head = root
        while head:
            ptr = head
            next_head = head.left
            # next layer is empty
            if next_head is None:
                break

            # link next layer
            #       ptr       ->   ptr.next
            #      /   \          /
            #   left -> right -> ptr.next.left   <== this level!
            while ptr:
                ptr.left.next = ptr.right
                if ptr.next:
                    ptr.right.next = ptr.next.left
                ptr = ptr.next
            head = next_head
        return root

# @lc code=end
