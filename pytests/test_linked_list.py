import rspy_ds as rspy

# Create linked list
ll = rspy.LinkedList()

# Expect linked list to be empty
assert len(ll) == 0

# push a few items
ll.push("a")
ll.push("b")
ll.push("c")

print(repr(ll))

# Indexing
assert ll[0] == "a"
assert ll[1] == "b"
assert ll[2] == "c"

# Size
assert len(ll) == 3

# Remove
assert ll.remove(0) == "a"
assert ll.remove(1) == "c"
assert ll.remove(0) == "b"

# Size
assert len(ll) == 0