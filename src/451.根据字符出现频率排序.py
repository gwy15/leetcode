#
# @lc app=leetcode.cn id=451 lang=python3
#
# [451] 根据字符出现频率排序
#

# @lc code=start
from collections import Counter


class Solution:
    def frequencySort(self, s: str) -> str:
        counter = Counter(s)
        ans = []

        for (ch, times) in counter.most_common():
            ans.append(ch * times)

        # chars = sorted(
        #     counter.keys(),
        #     key=lambda k: counter[k],
        #     reverse=True
        # )
        # for c in chars:
        #     ans.append(c * counter[c])
        return ''.join(ans)

# @lc code=end


if __name__ == "__main__":
    s = Solution()
    print(s.frequencySort('tree'))
