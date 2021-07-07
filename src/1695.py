from prelude import *


from collections import defaultdict

class Solution:
    def maximumUniqueSubarray(self, nums: List[int]) -> int:
        n = len(nums)
        counter = defaultdict(int)
        cur, best = 0, 0
        left, right = 0, 0
        while right < n:
            num = nums[right]
            counter[num] += 1
            cur += num
            

            if counter[num] <= 1:
                best = max(best, cur)
            while counter[num] > 1:
                # move left
                num_left = nums[left]
                cur -= num_left
                counter[num_left] -= 1
                left += 1
            right += 1
        return best
    

if __name__ == '__main__':
    def test(input, expected):
        calc = Solution().maximumUniqueSubarray(input)
        if calc != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test([4,2,4,5,6], 17)
    test([5,2,1,2,5,2,1,2,5], 8)