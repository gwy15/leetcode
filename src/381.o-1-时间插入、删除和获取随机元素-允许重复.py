#
# @lc app=leetcode.cn id=381 lang=python3
#
# [381] O(1) 时间插入、删除和获取随机元素 - 允许重复
#

import unittest
# @lc code=start
import random
from collections import defaultdict
from typing import Dict, Set


class RandomizedCollection:
    def __init__(self):
        """
        Initialize your data structure here.
        """
        self.items = []
        self.indexes: Dict[int, Set] = defaultdict(set)

    def insert(self, val: int) -> bool:
        """
        Inserts a value to the collection. Returns true if the collection did not already contain the specified element.
        """
        # print(f"insert {val}")
        already = val in self.indexes
        self.items.append(val)
        self.indexes[val].add(len(self.items) - 1)
        return not already

    def remove(self, val: int) -> bool:
        """
        Removes a value from the collection. Returns true if the collection contained the specified element.
        """
        # print(f"remove {val}, indexes={self.indexes}, items={self.items}")
        if val not in self.indexes:
            return False
        n = len(self.items)

        if self.items[-1] == val:
            self.indexes[val].remove(n-1)
            self.items.pop()
        else:
            i = self.indexes[val].pop()
            # swap i, n-1
            tail = self.items[n-1]
            self.items[i] = tail
            self.indexes[tail].remove(n-1)
            self.indexes[tail].add(i)
            # remove from items
            self.items.pop()
        if len(self.indexes[val]) == 0:
            del self.indexes[val]

        return True

    def getRandom(self) -> int:
        """
        Get a random element from the collection.
        """
        return random.choice(self.items)


# Your RandomizedCollection object will be instantiated and called as such:
# obj = RandomizedCollection()
# param_1 = obj.insert(val)
# param_2 = obj.remove(val)
# param_3 = obj.getRandom()
# @lc code=end


class Test(unittest.TestCase):
    def test(self):
        cmds = ["RandomizedCollection", "insert", "remove", "insert", "remove", "getRandom", "getRandom",
                "getRandom", "getRandom", "getRandom", "getRandom", "getRandom", "getRandom", "getRandom", "getRandom"]
        vals = [[], [0], [0], [-1], [0], [],
                [], [], [], [], [], [], [], [], []]

        s = RandomizedCollection()
        for i in range(1, len(cmds)):
            cmd = cmds[i]
            {
                'insert': lambda v: s.insert(v),
                'remove': lambda v: s.remove(v),
                'getRandom': lambda: s.getRandom()
            }[cmd](*vals[i])


if __name__ == "__main__":
    unittest.main()
