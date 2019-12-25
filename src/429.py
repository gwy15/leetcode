import queue


class Solution:
    def levelOrder(self, root: 'Node') -> List[List[int]]:
        if root is None:
            return []
        ans = []   # answer
        level = []  # this level
        q = queue.Queue()
        q.put(root)
        q.put(None)  # None marks a level end
        while not q.empty():
            node = q.get()
            if node is None:  # current level ends
                ans.append(level)  # save cur level to answer
                level = []  # restart recording
                if q.empty():  # no more levels
                    return ans
                q.put(None)  # mark next level ends
            else:
                level.append(node.val)  # record this level
                for child in node.children:
                    q.put(child)
