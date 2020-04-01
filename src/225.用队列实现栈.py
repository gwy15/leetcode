#
# @lc app=leetcode.cn id=225 lang=python3
#
# [225] 用队列实现栈
#

# @lc code=start
from queue import Queue


class MyStack:
    def __init__(self):
        """
        Initialize your data structure here.
        """
        self.queue = Queue()

    def len(self):
        return self.queue.qsize()

    def push(self, x: int) -> None:
        """
        Push element x onto stack. O(1)
        """
        return self.queue.put(x)

    def pop(self) -> int:
        """
        Removes the element on top of the stack and returns that element.
        """
        for i in range(self.len() - 1):
            self.queue.put(self.queue.get())
        return self.queue.get()

    def top(self) -> int:
        """
        Get the top element.
        """
        for i in range(self.len() - 1):
            self.queue.put(self.queue.get())
        ans = self.queue.get()
        self.queue.put(ans)
        return ans

    def empty(self) -> bool:
        """
        Returns whether the stack is empty.
        """
        return self.queue.empty()

# @lc code=end


# Your MyStack object will be instantiated and called as such:
obj = MyStack()
obj.push(1)
param_2 = obj.pop()
param_3 = obj.top()
param_4 = obj.empty()
