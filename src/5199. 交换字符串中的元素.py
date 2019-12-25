from typing import List


class Solution:
    def smallestStringWithSwaps(self, s: str, pairs: List[List[int]]) -> str:
        s = list(s)

        keySet = []  # [{posIndex}]
        posToSetIndexes = {}  # pos -> index in keySet
        for a, b in pairs:
            indexA = posToSetIndexes.get(a, None)
            indexB = posToSetIndexes.get(b, None)
            if indexA is not None and indexB is None:  # a only
                keySet[indexA].add(b)
                posToSetIndexes[b] = indexA
            elif indexA is None and indexB is not None:  # b only
                keySet[indexB].add(a)
                posToSetIndexes[a] = indexB
            elif indexA is None and indexB is None:  # both none
                index = len(keySet)
                keySet.append({a, b})
                posToSetIndexes[a] = index
                posToSetIndexes[b] = index
            elif indexA == indexB:
                continue
            else:  # merge B into A
                if len(keySet[indexA]) < len(keySet[indexB]):
                    indexA, indexB = indexB, indexA
                setA = keySet[indexA]
                setB = keySet[indexB]
                keySet[indexB] = set()
                for key in setB:  # point to B
                    posToSetIndexes[key] = indexA
                setA |= setB

        for strIndexSet in keySet:
            sortedPartialStr = sorted(s[i] for i in strIndexSet)
            # put back
            for partialStrIndex, pos in enumerate(sorted(strIndexSet)):
                s[pos] = sortedPartialStr[partialStrIndex]

        return ''.join(s)


# print(Solution().smallestStringWithSwaps('dcab', [[0, 3], [1, 2]]))
# print(Solution().smallestStringWithSwaps('dcab', [[0, 3], [1, 2], [0, 2]]))
# print(Solution().smallestStringWithSwaps('cba', [[0, 1], [1, 2]]))
# print('ffkqttkv')
# print(Solution().smallestStringWithSwaps("fqtvkfkt", [
#       [2, 4], [5, 7], [1, 0], [0, 0], [4, 7], [0, 3], [4, 1], [1, 3]]))
print(Solution().smallestStringWithSwaps("yhiihxbordwyjybyt",
                                         [[9, 1], [5, 11], [9, 7], [2, 7], [14, 16], [6, 16], [0, 5], [12, 9], [6, 5], [9, 10], [4, 7], [3, 2], [10, 1], [3, 15], [12, 4], [10, 10], [15, 12]]))
