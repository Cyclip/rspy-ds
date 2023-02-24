//! A binary tree data structure

use pyo3::{prelude::*};

type TreeLink = Option<Box<TreeNode>>;

/// Tree
#[pyclass]
pub struct Tree {
    root: TreeLink,
}

#[pyclass]
pub struct TreeNode {
    elem: PyObject,
    left: TreeLink,
    right: TreeLink,
}

#[pymethods]
impl Tree {
    #[new]
    fn new() -> Self {
        Tree { root: None }
    }

    #[getter(root)]
    fn get_root(&self) -> Option<PyObject> {
        if let Some(root) = self.root.as_ref() {
            Python::with_gil(|py| {
                Some(TreeNode::itself(
                    root,
                    py
                ).unwrap())
            })
        } else {
            None
        }
    }

    /// Set root node
    fn set_root(&mut self, elem: PyObject) -> PyResult<PyObject> {
        let new_node = Box::new(TreeNode {
            elem: elem,
            left: None,
            right: None,
        });

        self.root = Some(new_node);

        Ok(self.root.as_ref().unwrap().elem.clone())
    }

    /// Depth-first search algorithm
    fn dfsearch(&self, target: PyObject) -> Option<PyObject> {
        // Check if the root node is the target
        if self.root.as_ref().map(|node| {
            node.elem.is(&target)
        }).unwrap_or(false) {
            return Some(self.root.as_ref().unwrap().elem.clone());
        }

        // Check if the left child is the target
        if self.root.as_ref().map(|node| {
            node.left.as_ref().map(|child| {
                child.elem.is(&target)
            }).unwrap_or(false)
        }).unwrap_or(false) {
            return Some(self.root.as_ref().unwrap().left.as_ref().unwrap().elem.clone());
        }

        // Check if the right child is the target
        if self.root.as_ref().map(|node| {
            node.right.as_ref().map(|child| {
                child.elem.is(&target)
            }).unwrap_or(false)
        }).unwrap_or(false) {
            return Some(self.root.as_ref().unwrap().right.as_ref().unwrap().elem.clone());
        }

        None
    }
}

/// Private methods
impl Tree {
    fn itself(slf: PyRef<Self>, py: Python) -> PyResult<PyObject> {
        Ok(slf.into_py(py))
    }
}

#[pymethods]
impl TreeNode {
    /// Indexing
    fn __getitem__(&self, target: PyObject) -> Option<PyObject> {
        Python::with_gil(|py| {
            // Is left child?
            if let Some(left) = self.left.as_ref() {
                if left.elem.is(&target) {
                    return Some(left.elem.clone_ref(py));
                }
            }

            // Is right child?
            if let Some(right) = self.right.as_ref() {
                if right.elem.is(&target) {
                    return Some(right.elem.clone_ref(py));
                }
            }

            None
        })
    }

    /// Add a child node
    fn add_child(&mut self, elem: PyObject) -> PyResult<()> {
        let new_node = Box::new(TreeNode {
            elem: elem,
            left: None,
            right: None,
        });

        if self.left.is_none() {
            self.left = Some(new_node);
            Ok(())
        } else if self.right.is_none() {
            self.right = Some(new_node);
            Ok(())
        } else {
            // return py error
            Err(
                PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    "Cannot add more than two children"
                )
            )
        }
    }
}

/// Private methods
impl TreeNode {
    fn itself(slf: PyRef<Self>, py: Python) -> PyResult<PyObject> {
        Ok(slf.into_py(py))
    }
}