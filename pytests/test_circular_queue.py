import rspy_ds as rspy

# Create circular queue
cq = rspy.CircularQueue(4)

# Expect IndexError to be raised when trying to dequeue from empty queue
try:
    cq.dequeue()
except IndexError:
    pass

# Add items to queue
cq.enqueue(1)
print(f"Added 1 to queue")
cq.enqueue(2)
print(f"Added 2 to queue")
cq.enqueue(3)
print(f"Added 3 to queue")
cq.enqueue(4)
print(f"Added 4 to queue")

# Dequeue
assert cq.dequeue() == 1
print(f"Dequeued 1 from queue")
assert cq.dequeue() == 2
print(f"Dequeued 2 from queue")

# Add items to queue
cq.enqueue(5)
print(f"Added 5 to queue")
cq.enqueue(6)
print(f"Added 6 to queue")

# Dequeue
assert cq.dequeue() == 3
print(f"Dequeued 3 from queue")
assert cq.dequeue() == 4
print(f"Dequeued 4 from queue")
assert cq.dequeue() == 5
print(f"Dequeued 5 from queue")
assert cq.dequeue() == 6
print(f"Dequeued 6 from queue")