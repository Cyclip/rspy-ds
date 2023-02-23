import rspy_ds as rspy

# Create queue
q = rspy.Queue(4)

# Expect queue to be empty
assert q.is_empty() == True

# Add items to queue
q.enqueue(1)
q.enqueue(2)

# Expect queue to have length of 2
assert q.len() == 2

q.enqueue(3)
q.enqueue(4)

# Expect queue to be full
assert q.is_full() == True

# Dequeue
assert q.dequeue() == 1
assert q.dequeue() == 2
assert q.dequeue() == 3
assert q.dequeue() == 4