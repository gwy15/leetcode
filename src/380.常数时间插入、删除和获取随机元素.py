#
# @lc app=leetcode.cn id=380 lang=python3
#
# [380] 常数时间插入、删除和获取随机元素
#

# @lc code=start
import random


class RandomizedSet:

    def __init__(self):
        """
        Initialize your data structure here.
        """
        self.map = dict()
        self.items = list()

    def insert(self, val: int) -> bool:
        """
        Inserts a value to the set. Returns true if the set did not already contain the specified element.
        """
        if val in self.map:
            return False
        n = len(self.items)
        self.items.append(val)
        self.map[val] = n
        return True

    def remove(self, val: int) -> bool:
        """
        Removes a value from the set. Returns true if the set contained the specified element.
        """
        if val not in self.map:
            return False
        i = self.map[val]
        n = len(self.items)
        if i != n - 1:
            x = self.items[n - 1]
            # swap items
            self.items[i] = x
            self.items[n-1] = val
            # swap map
            self.map[x] = i
            self.map[val] = n-1

        del self.map[val]
        self.items.pop()
        return True

    def getRandom(self) -> int:
        """
        Get a random element from the set.
        """
        return random.choice(self.items)


# Your RandomizedSet object will be instantiated and called as such:
# @lc code=end

obj = RandomizedSet()
assert obj.insert(1)
assert obj.remove(2) is False
obj.getRandom()
