#
# @lc app=leetcode.cn id=318 lang=python3
#
# [318] 最大单词长度乘积
#

# @lc code=start
class Solution:
    def maxProduct(self, words: List[str]) -> int:
        def word_to_mask(word):
            mask = 0
            for w in word:
                mask |= 1 << (ord(w) - ord('a'))
            return mask

        n = len(words)
        mask = [0] * n
        for i in range(n):
            mask[i] = word_to_mask(words[i])
        
        ans = 0
        for i in range(n):
            for j in range(i+1, n):
                if mask[i] & mask[j] == 0:
                    ans = max(ans, len(words[i]) * len(words[j]))
        return ans

        
# @lc code=end

