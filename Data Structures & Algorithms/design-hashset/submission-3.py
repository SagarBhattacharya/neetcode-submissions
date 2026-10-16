class MyHashSet:

    def __init__(self):
        self.capacity = 1000
        self.data = [[] for _ in range(self.capacity)]

    def add(self, key: int) -> None:
        k = key % self.capacity
        if key not in self.data[k]:
            self.data[k].append(key)

    def remove(self, key: int) -> None:
        k = key % self.capacity
        if key in self.data[k]:
            self.data[k].remove(key)

    def contains(self, key: int) -> bool:
        k = key % self.capacity
        return key in self.data[k]


# Your MyHashSet object will be instantiated and called as such:
# obj = MyHashSet()
# obj.add(key)
# obj.remove(key)
# param_3 = obj.contains(key)