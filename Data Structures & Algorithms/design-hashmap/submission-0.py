class MyHashMap:

    def __init__(self):
        self.capacity = 1024
        self.data = [[] for _ in range(self.capacity)]

    def put(self, key: int, value: int) -> None:
        bucket = self.data[key % self.capacity]
        for i, (k, v) in enumerate(bucket):
            if k == key:
                bucket[i] = (k, value)
                return
        bucket.append((key, value))

    def get(self, key: int) -> int:
        bucket = self.data[key % self.capacity]
        for i, (k, v) in enumerate(bucket):
            if k == key:
                return v
        return -1

    def remove(self, key: int) -> None:
        bucket = self.data[key % self.capacity]
        for i, (k, v) in enumerate(bucket):
            if k == key:
                del bucket[i]
                break


# Your MyHashMap object will be instantiated and called as such:
# obj = MyHashMap()
# obj.put(key,value)
# param_2 = obj.get(key)
# obj.remove(key)