#
# @lc app=leetcode.cn id=430 lang=python3
#
# [430] 扁平化多级双向链表
#

class Node:
    def __init__(self, val, prev, next, child):
        self.val = val
        self.prev = prev
        self.next = next
        self.child = child
# @lc code=start


class Solution:
    def flatten_helper(self, head: 'Node') -> ('Node', 'Node'):
        assert head is not None
        ptr = head

        while True:
            # need flat
            if ptr.child is not None:
                next = ptr.next
                child_head, child_tail = self.flatten_helper(ptr.child)
                ptr.child = None
                if next is None:
                    ptr.next, child_head.prev = child_head, ptr
                    ptr = child_tail
                    break
                else:
                    ptr.next, child_head.prev = child_head, ptr
                    next.prev, child_tail.next = child_tail, next
                    ptr = child_tail

            if ptr.next is None:
                break
            else:
                ptr = ptr.next
        return head, ptr

    def flatten(self, head: 'Node') -> 'Node':
        if head is None:
            return None
        return self.flatten_helper(head)[0]


# @lc code=end
if __name__ == '__main__':
    nodes = {i: Node(i, None, None, None) for i in range(1, 13)}

    def c(i, j):
        nodes[i].next, nodes[j].prev = nodes[j], nodes[i]
    c(1, 2)
    c(2, 3)
    c(3, 4)
    c(4, 5)
    c(5, 6)

    c(7, 8)
    c(8, 9)
    c(9, 10)

    c(11, 12)

    def d(i, j):
        nodes[i].child = nodes[j]

    d(3, 7)
    d(8, 11)

    Solution().flatten(nodes[1])

    def a_l(i, j):
        assert nodes[i].next == nodes[j]
        assert nodes[j].prev == nodes[i]
    arr = [1, 2, 3, 7, 8, 11, 12, 9, 10, 4, 5, 6]
    for i in range(len(arr) - 1):
        a_l(arr[i], arr[i+1])
