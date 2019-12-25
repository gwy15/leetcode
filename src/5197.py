from typing import List

class Solution:
    def minimumAbsDifference(self, arr: List[int]) -> List[List[int]]:
        if len(arr) <= 1:
            return []
        arr.sort()
        diff = [arr[i+1] - arr[i] for i in range(len(arr)-1)]
        minDiff = min(diff)
        results = []
        for i in range(len(arr)-1):
            if arr[i+1] - arr[i] == minDiff:
                results.append([arr[i], arr[i+1]])
        return results
        
print(Solution().minimumAbsDifference([4,2,1,3]))
print(Solution().minimumAbsDifference([1,3,6,10,15]))
print(Solution().minimumAbsDifference([3,8,-10,23,19,-4,-14,27]))
