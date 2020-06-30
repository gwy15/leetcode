# from queue import Queue


class CQueue:

    def __init__(self):
        # self.input = Queue()
        # self.output = Queue()
        self.input = []
        self.output = []

    def appendTail(self, value: int) -> None:
        # self.input.put(value)
        self.input.append(value)

    def deleteHead(self) -> int:
        if self.output:
            return self.output.pop()
        if not self.input:
            return -1
        n = len(self.input)
        for _ in range(n):
            self.output.append(self.input.pop())
        return self.output.pop()
        # if self.output.qsize():
        #     return self.output.get()
        # if self.input.qsize() == 0:
        #     return -1

        # # move buffer from input to output
        # n = self.input.qsize()
        # for _ in range(n):
        #     v = self.input.get()
        #     self.output.put(v)
        # return self.output.get()
