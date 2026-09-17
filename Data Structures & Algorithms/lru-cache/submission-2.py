class Node:
    def __init__(self, key=0, val=0):
        self.key = key
        self.val = val
        self.next = None
        self.prev = None

class LRUCache:

    def __init__(self, capacity: int):
        self.capacity = capacity
        self.data = {}

        self.lru = Node()
        self.mru = Node()

        self.lru.next = self.mru
        self.mru.prev = self.lru

    def remove(self, node):
        node.prev.next = node.next
        node.next.prev = node.prev

    def insert_tail(self, node):
        node.prev = self.mru.prev
        node.next = self.mru

        node.prev.next = node
        node.next.prev = node

    def get(self, key: int) -> int:
        if key not in self.data:
            return -1

        node = self.data[key]
        self.remove(node)
        self.insert_tail(node)
        return node.val

    def put(self, key: int, value: int) -> None:
        if key in self.data:
            node = self.data[key]
            node.val = value

            self.remove(node)
            self.insert_tail(node)

            return

        if len(self.data) >= self.capacity:
            lru = self.lru.next
            self.remove(lru)
            self.data.pop(lru.key)

        node = Node(key, value)
        self.data[key] = node
        self.insert_tail(node)
