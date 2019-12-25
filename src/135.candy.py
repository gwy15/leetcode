#
# @lc app=leetcode id=135 lang=python3
#
# [135] Candy
#
class Solution:
    def candy(self, ratings: List[int]) -> int:
        length = len(ratings)
        candies = [1] * length
        for i in range(length-1):
            if ratings[i] < ratings[i+1]:
                candies[i+1] = max(candies[i+1], candies[i]+1)
        for i in range(length-1, 0, -1):
            if ratings[i-1] > ratings[i]:
                candies[i-1] = max(candies[i-1], candies[i]+1)
        return sum(candies)
