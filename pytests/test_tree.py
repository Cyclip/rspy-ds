import rspy_ds as rspy

# Create a tree
tree = rspy.Tree()

# Create root node
tree.set_root("root")

# Create child nodes
print(f"Types: {tree.get_root()} {type(tree.get_root())}")
tree.get_root().add_child("child1")
tree.get_root().add_child("child2")

# Create grandchild nodes
tree.get_root()["child1"].add_child("grandchild1")
tree.get_root()["child1"].add_child("grandchild2")

# depth-first search
print(tree.dfsearch("grandchild1"))

# breadth-first search
print(tree.bfsearch("grandchild2"))