#
# @lc app=leetcode.cn id=676 lang=python3
#
# [676] 实现一个魔法字典
#
from typing import List
# @lc code=start
from collections import defaultdict


class MagicDictionary:

    def __init__(self):
        """
        Initialize your data structure here.
        """
        self.words = defaultdict(set)

    def buildDict(self, dict: List[str]) -> None:
        """
        Build a dictionary through a list of words
        """
        for word in dict:
            self.words[len(word)].add(word)

    def search(self, word: str) -> bool:
        """
        Returns if there is any word in the trie that equals to the given word after modifying exactly one character
        """
        n = len(word)
        words = self.words[n]
        for w in words:
            if sum(w[i] != word[i] for i in range(n)) == 1:
                return True
        return False


# Your MagicDictionary object will be instantiated and called as such:
# @lc code=end

obj = MagicDictionary()
obj.buildDict(["hello", "leetcode"])
assert obj.search('hello') == False
assert obj.search('hhllo') == True
