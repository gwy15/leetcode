#
# @lc app=leetcode.cn id=933 lang=python3
#
# [933] 最近的请求次数
#
from typing import List
from utils import *
# @lc code=start
from collections import deque


class RecentCounter:
    def __init__(self):
        self.q = deque()

    def ping(self, t: int) -> int:
        self.q.append(t)
        while self.q[0] < (t - 3000):
            self.q.popleft()
        return len(self.q)


# Your RecentCounter object will be instantiated and called as such:
# obj = RecentCounter()
# param_1 = obj.ping(t)
# @lc code=end
