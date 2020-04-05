#
# @lc app=leetcode.cn id=460 lang=python3
#
# [460] LFU缓存
#
import unittest
# @lc code=start
from typing import Dict, Tuple, Optional


class LinkedList:
    def __init__(self):
        self.left: Optional[LinkedList] = None
        self.right: Optional[LinkedList] = None

    @staticmethod
    def hook(left, right):
        left.right = right
        right.left = left

    def strip(self):
        LinkedList.hook(self.left, self.right)
        self.left, self.right = None, None

    @classmethod
    def new(cls):
        # head = cls(None)
        # tail = cls(None)
        head = cls('head')
        tail = cls('tail')
        LinkedList.hook(head, tail)
        return head, tail

    @staticmethod
    def stringify(head: 'LinkedList', tail: 'LinkedList'):
        def nodes():
            p = head
            while p is not None:
                yield str(p)
                p = p.right
        return ' <-> '.join(nodes())


class BoxNode(LinkedList):
    def __init__(self, key, value=''):
        super().__init__()
        self.key = key
        self.value = value

    def __repr__(self):
        return f'<Node {self.key}: {self.value}>'


class CountBox(LinkedList):
    def __init__(self, count):
        super().__init__()
        self.count = count
        self.node_count = 0
        self.head, self.tail = BoxNode.new()

    def push(self, node: BoxNode):
        first = self.head.right
        LinkedList.hook(self.head, node)
        LinkedList.hook(node, first)
        self.node_count += 1

    def pop(self) -> BoxNode:
        # first = self.head.right
        node = self.tail.left
        node.strip()
        self.node_count -= 1
        return node

    def __repr__(self):
        nodes = LinkedList.stringify(self.head, self.tail)
        return f'\n<Box {self.count} nodes=({self.node_count}) ({nodes})>\n'


class LFUCache:
    def __init__(self, capacity: int):
        self.capacity = capacity
        self.size = 0
        self.head, self.tail = CountBox.new()
        self.mapper: Dict[int, Tuple[CountBox, BoxNode]] = dict()

    def get(self, key: int) -> int:
        if self.capacity == 0:
            return -1
        if key not in self.mapper:
            return -1
        box, node = self.mapper[key]
        # remove node from box
        if box.node_count == 1:
            if box.right.count == box.count + 1:
                # destroy this box and insert into right box
                node.strip()
                # this should do the GC
                LinkedList.hook(box.left, box.right)
                box = box.right
                box.push(node)
            else:
                # yes! simply increase the count!
                box.count += 1
        else:
            # box not empty, first remove node from the box
            node.strip()
            box.node_count -= 1
            if box.right.count == box.count + 1:
                # insert into next box
                box = box.right
                box.push(node)
            else:
                # insert new box
                new_box = CountBox(box.count + 1)
                right = box.right
                LinkedList.hook(box, new_box)
                LinkedList.hook(new_box, right)
                box = new_box
                box.push(node)
        # save to mapper
        self.mapper[key] = box, node
        return node.value

    def put(self, key: int, value: int) -> None:
        if self.capacity == 0:
            return
        if key not in self.mapper:
            self.insert(key, value)
        else:
            self.mapper[key][1].value = value
            # update freq count
            self.get(key)

    def expel(self):
        first_box = self.head.right
        # first box must not be empty, so just do the pop
        node = first_box.pop()
        if first_box.node_count == 0:
            # remove the first box as well
            first_box.strip()
        # expel node
        self.size -= 1
        del self.mapper[node.key]

    def insert(self, key, value):
        if self.size == self.capacity:
            self.expel()
        # insert
        node = BoxNode(key, value)
        # try the first box
        first_box = self.head.right
        if first_box.count == 1:
            # this is the one
            box = first_box
            box.push(node)
        else:
            # insert new one-box
            box = CountBox(1)
            LinkedList.hook(self.head, box)
            LinkedList.hook(box, first_box)
            box.push(node)

        self.size += 1
        self.mapper[key] = box, node

    def debug_print(self):
        print(f'size: {self.size}', end='')
        print(LinkedList.stringify(self.head, self.head))

# Your LFUCache object will be instantiated and called as such:
# obj = LFUCache(capacity)
# param_1 = obj.get(key)
# obj.put(key,value)
# @lc code=end


class LFUTestCase(unittest.TestCase):
    def test1(self):
        c = LFUCache(2)
        c.put(1, 1)
        c.put(2, 2)
        self.assertEqual(c.get(1), 1)
        c.put(3, 3)
        self.assertEqual(c.get(2), -1)
        self.assertEqual(c.get(3), 3)
        c.put(4, 4)
        self.assertEqual(c.get(1), -1)
        self.assertEqual(c.get(3), 3)
        self.assertEqual(c.get(4), 4)

    def test2(self):
        c = LFUCache(2)
        c.put(3, 1)
        c.put(2, 1)
        c.put(2, 2)
        c.put(4, 4)
        self.assertEqual(c.get(2), 2)

    def test3(self):
        c = LFUCache(0)
        c.put(0, 0)
        self.assertEqual(c.get(0), -1)


if __name__ == "__main__":
    unittest.main()
