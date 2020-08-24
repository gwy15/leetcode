#
# @lc app=leetcode.cn id=641 lang=python3
#
# [641] 设计循环双端队列
#

# @lc code=start
class DequeNode:
    def __init__(self, v: int):
        self.val = v
        self.left: DequeNode = None
        self.right: DequeNode = None

    def trim(self):
        self.left = None
        self.right = None


def link(left: DequeNode, right: DequeNode):
    left.right = right
    right.left = left


class MyCircularDeque:

    def __init__(self, k: int):
        """
        Initialize your data structure here. Set the size of the deque to be k.
        """
        self.maxsize = k
        self.size = 0
        self.head = DequeNode(-1)
        self.tail = DequeNode(-1)
        link(self.head, self.tail)

    def insertFront(self, value: int) -> bool:
        """
        Adds an item at the front of Deque. Return true if the operation is successful.
        """
        if self.isFull():
            return False
        node = DequeNode(value)
        next = self.head.right
        link(self.head, node)
        link(node, next)
        self.size += 1
        return True

    def insertLast(self, value: int) -> bool:
        """
        Adds an item at the rear of Deque. Return true if the operation is successful.
        """
        if self.isFull():
            return False
        node = DequeNode(value)
        before = self.tail.left
        link(before, node)
        link(node, self.tail)
        self.size += 1
        return True

    def deleteFront(self) -> bool:
        """
        Deletes an item from the front of Deque. Return true if the operation is successful.
        """
        if self.isEmpty():
            return False
        node = self.head.right
        link(self.head, node.right)
        node.trim()
        self.size -= 1
        return True

    def deleteLast(self) -> bool:
        """
        Deletes an item from the rear of Deque. Return true if the operation is successful.
        """
        if self.isEmpty():
            return False
        node = self.tail.left
        link(node.left, self.tail)
        node.trim()
        self.size -= 1
        return True

    def getFront(self) -> int:
        """
        Get the front item from the deque.
        """
        return self.head.right.val

    def getRear(self) -> int:
        """
        Get the last item from the deque.
        """
        return self.tail.left.val

    def isEmpty(self) -> bool:
        """
        Checks whether the circular deque is empty or not.
        """
        return self.size == 0

    def isFull(self) -> bool:
        """
        Checks whether the circular deque is full or not.
        """
        return self.size == self.maxsize


# Your MyCircularDeque object will be instantiated and called as such:
# obj = MyCircularDeque(k)
# param_1 = obj.insertFront(value)
# param_2 = obj.insertLast(value)
# param_3 = obj.deleteFront()
# param_4 = obj.deleteLast()
# param_5 = obj.getFront()
# param_6 = obj.getRear()
# param_7 = obj.isEmpty()
# param_8 = obj.isFull()
# @lc code=end
