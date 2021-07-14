#
# @lc app=leetcode.cn id=950 lang=python3
#
# [950] 按递增顺序显示卡牌
#
from prelude import *
# @lc code=start

from collections import deque


class Solution:
    def deckRevealedIncreasing(self, deck: List[int]) -> List[int]:
        n = len(deck)
        q = deque(i for i in range(n))
        # 建立一个映射：{第i次抽出来的牌 / 第i小的牌} => {原有位置}
        mapper = []
        for i in range(n):
            kth_small = q.popleft()
            mapper.append(kth_small)
            if q:
                q.append(q.popleft())

        # 反向映射，将 {原有位置} => {第i次抽出来的牌/第i小的牌}
        rev_mapper = [-1 for i in range(n)]
        for i in range(n):
            rev_mapper[mapper[i]] = i

        deck = sorted(deck)
        # 将 deck 填到 ans
        ans = [
            deck[rev_mapper[i]]
            for i in range(n)
        ]
        return ans


# @lc code=end
if __name__ == '__main__':
    def test(input, expected):
        calc = Solution().deckRevealedIncreasing(input)
        if calc != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test([17, 13, 11, 2, 3, 5, 7], [2, 13, 3, 11, 5, 17, 7])
    test([1, 2, 3], [1, 3, 2])
    test([1, ], [1])
