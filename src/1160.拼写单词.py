#
# @lc app=leetcode.cn id=1160 lang=python3
#
# [1160] 拼写单词
#

# @lc code=start
from collections import Counter
class Solution:
    def countCharacters(self, words: List[str], chars: str) -> int:
        c = Counter(chars)
        res = 0
        for w in  words:
            wc = Counter(w)
            ok = True
            for ch in wc:
                if wc[ch] > c[ch]:
                    ok = False
                    break
            if ok:
                res += len(w)
        return res
        
# @lc code=end

