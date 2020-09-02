from typing import List
from utils import *

from collections import defaultdict


class Solution:
    def pairSums(self, nums: List[int], target: int) -> List[List[int]]:
        ans = []
        counter = defaultdict(int)
        for n in nums:
            pair = target - n
            pair_cnt = counter[pair]
            if pair_cnt >= 1:
                if pair_cnt == 1:
                    counter.pop(pair)
                else:
                    counter[pair] -= 1
                ans.append([pair, n])
            else:
                counter[n] += 1
        return ans
