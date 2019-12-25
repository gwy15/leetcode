#
# @lc app=leetcode id=432 lang=python3
#
# [432] All O`one Data Structure
#

# @lc code=start


class AllOne:

    def __init__(self):
        """
        Initialize your data structure here.
        """

        self.countMap = {}  # key -> count
        self.buckets = [set()]  # List[set[str]] count -> set

    def getBucket(self, count):
        if len(self.buckets) > count:
            return self.buckets[count]
        else:
            assert len(self.buckets) == count
            self.buckets.append(set())
            return self.buckets[count]

    def inc(self, key: str) -> None:
        """
        Inserts a new key <Key> with value 1. Or increments an existing key by 1.
        """
        if key not in self.countMap:  # new, insert
            self.countMap[key] = 1
            self.getBucket(1).add(key)
        else:
            count = self.countMap[key]
            self.countMap[key] += 1
            self.getBucket(count).remove(key)
            self.getBucket(count+1).add(key)

    def dec(self, key: str) -> None:
        """
        Decrements an existing key by 1. If Key's value is 1, remove it from the data structure.
        """
        if key not in self.countMap:
            return  # do nothing

        count = self.countMap[key]
        if count == 1:
            self.getBucket(count).remove(key)
            del self.countMap[key]
        else:
            self.countMap[key] -= 1
            self.getBucket(count).remove(key)
            self.getBucket(count-1).add(key)

    def getMaxKey(self) -> str:
        """
        Returns one of the keys with maximal value.
        """
        if len(self.buckets) == 1:
            return ''
        return next(self.buckets[-1])

    def getMinKey(self) -> str:
        """
        Returns one of the keys with Minimal value.
        """
        if len(self.buckets) == 1:
            return ''
        return next(self.buckets[1])


# Your AllOne object will be instantiated and called as such:
# obj = AllOne()
# obj.inc(key)
# obj.dec(key)
# param_3 = obj.getMaxKey()
# param_4 = obj.getMinKey()
# @lc code=end
