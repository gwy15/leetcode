#
# @lc app=leetcode.cn id=139 lang=python3
#
# [139] 单词拆分
#
from typing import List
# @lc code=start
import queue
class TreeNode:
    def __init__(self, is_node: bool):
        self.is_node = is_node
        self.children = [None] * 26

    def __str__(self):
        children = []
        for idx in range(26):
            if self.children[idx] is not None:
                children.append('{}:{}'.format(
                    chr(ord('a') + idx),
                    self.children[idx]
                ))
        return f'<{self.is_node} children=[{", ".join(children)}]>'    

class Solution:
    def wordBreak(self, s: str, wordDict: List[str]) -> bool:
        # generate prefix tree
        root = TreeNode(False)
        for word in wordDict:
            node = root
            for ch in word:
                idx = ord(ch) - ord('a')
                if node.children[idx] is None:
                    node.children[idx] = TreeNode(False)
                node = node.children[idx]
            node.is_node = True
        
        n = len(s)
        dp = [False] * (n + 1)
        q = queue.Queue()
        dp[0] = True
        q.put(0)
        while q.qsize():
            idx = q.get()
            # iterate thru [idx, n)
            i = idx
            node = root
            while i < n:
                ch = ord(s[i]) - ord('a')
                if node.children[ch] is None:
                    break
                node = node.children[ch]
                i += 1
                if node.is_node:
                    if dp[i] == False:
                        dp[i] = True
                        q.put(i)
        return dp[n]


# @lc code=end

if __name__ == "__main__":
    s = Solution()
    assert not s.wordBreak('asdf', ['as', 'a', 'asde'])
    assert s.wordBreak('leetcode', ['leet', 'code'])
    assert s.wordBreak('applepenapple', ['apple', 'pen'])
    assert not s.wordBreak('catsandog', ['cats', 'dog', 'sand', 'and', 'cat'])
    assert s.wordBreak('', ['a'])

