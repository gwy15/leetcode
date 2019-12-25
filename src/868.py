class Solution:
    def binaryGap(self, N: int) -> int:
        pos = 0
        lastPos = -1
        maxLength = 0
        while N:
            isOne = N & 1
            if isOne:
                if lastPos != -1:
                    maxLength = max(maxLength, pos - lastPos)
                lastPos = pos
            N >>= 1
            pos += 1
        return maxLength