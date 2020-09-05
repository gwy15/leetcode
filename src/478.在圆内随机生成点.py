#
# @lc app=leetcode.cn id=478 lang=python3
#
# [478] 在圆内随机生成点
#
from typing import List
from utils import *
from matplotlib import pyplot as plt
# @lc code=start
import random
import math


class Solution:

    def __init__(self, radius: float, x_center: float, y_center: float):
        self.x, self.y, self.r = x_center, y_center, radius

    def randPoint(self) -> List[float]:
        theta = random.random() * 2.0 * math.pi
        r = math.sqrt(random.random())
        return [
            self.x + self.r * r * math.cos(theta),
            self.y + self.r * r * math.sin(theta)
        ]


# Your Solution object will be instantiated and called as such:
# obj = Solution(radius, x_center, y_center)
# param_1 = obj.randPoint()
# @lc code=end
if __name__ == "__main__":
    s = Solution(1, 0, 0)
    x, y = [], []
    for _ in range(1000):
        p = s.randPoint()
        x.append(p[0])
        y.append(p[1])
    plt.scatter(x, y)
    plt.show()
