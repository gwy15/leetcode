#
# @lc app=leetcode.cn id=763 lang=python3
#
# [763] 划分字母区间
#
from typing import List
from utils import *
# @lc code=start


class Solution:
    def partitionLabels(self, S: str) -> List[int]:
        first, last = {}, {}
        for i, ch in enumerate(S):
            if ch not in first:
                first[ch] = i
            last[ch] = i
        # sort
        regions = []
        for ch in first.keys():
            regions.append((first[ch], last[ch]))
        regions.sort()
        # make lengths
        lengths = []
        first, last = 0, 0
        i = 0
        while i < len(regions):
            while i < len(regions) and regions[i][0] <= last:
                last = max(last, regions[i][1])
                i += 1
            #
            lengths.append(last - first + 1)
            first = last + 1
            last = last + 1

        return lengths


# @lc code=end
if __name__ == '__main__':
    def test(input, expected):
        calc = Solution().partitionLabels(input)
        if calc != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test('aba', [3])
    test('abace', [3, 1, 1])
    test('abcbeffffe', [1, 3, 6])
    test("ababcbacadefegdehijhklij", [9, 7, 8])
    test('', [])
