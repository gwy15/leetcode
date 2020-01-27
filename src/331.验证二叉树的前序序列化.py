class Solution:
    def isValidSerialization(self, preorder: str) -> bool:
        # print(preorder)
        current_expected = 1
        current = 0
        next_expected = 0

        nodes = preorder.split(',')
        for index, node in enumerate(nodes):
            if node == '#':
                current += 1
            else:  # valid
                current += 1
                next_expected += 2

            if current == current_expected:  # next
                # print(f'current level ok, next expect: {next_expected}')
                current_expected = next_expected
                current = 0
                next_expected = 0
                if current_expected == 0:
                    return index == len(nodes) - 1
        return False


if __name__ == "__main__":
    assert(Solution().isValidSerialization(
        "9,3,4,#,#,1,#,#,2,#,6,#,#"
    ) == True)
    
    assert(Solution().isValidSerialization(
        "1,#,#,1"
    ) == False)
