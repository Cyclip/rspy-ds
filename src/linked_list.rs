use pyo3::{prelude::*, exceptions::PyIndexError};

type Link = Option<Box<Node>>;

/// Linked list
#[pyclass]
pub struct LinkedList {
    head: Link,
    size: usize,
}

#[pyclass]
pub struct Node {
    elem: PyObject,
    next: Link
}

#[pymethods]
impl LinkedList {
    #[new]
    fn new() -> Self {
        LinkedList { head: None, size: 0 }
    }

    /// Push an element onto the front of the list
    fn push(&mut self, elem: PyObject) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);

        self.size += 1;
    }

    /// Pop an element off the front of the list
    fn pop(&mut self) -> Option<PyObject> {
        let rv = self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        });

        self.size -= 1;
        rv
    }

    /// Remove index from the list
    fn remove(&mut self, idx: usize) -> PyResult<PyObject> {
        let idx = self.reverse_index(idx).unwrap();
        if idx >= self.size {
            return Err(PyIndexError::new_err("list index out of range"));
        }

        let mut cur_link = &mut self.head;
        for _ in 0..idx {
            cur_link = &mut cur_link.as_mut().unwrap().next;
        }

        let rv = cur_link.take().map(|node| {
            *cur_link = node.next;
            node.elem
        });

        self.size -= 1;
        Ok(rv.unwrap())
    }

    /// Print the list
    fn __repr__(&self) -> PyResult<String> {
        let mut s = String::from("LinkedList(");
        let mut cur_link = &self.head;
        while let Some(boxed_node) = cur_link {
            s.push_str(&format!("{:?}, ", boxed_node.elem.to_string()));
            cur_link = &boxed_node.next;
        }
        s.push(')');
        Ok(s)
    }

    /// Get the length of the list
    fn __len__(&self) -> PyResult<usize> {
        Ok(self.size)
    }

    /// Index into the list
    fn __getitem__(&self, idx: usize) -> PyResult<PyObject> {
        let idx = self.reverse_index(idx).unwrap();
        
        if idx >= self.size {
            return Err(PyIndexError::new_err("list index out of range"));
        }

        let mut cur_link = &self.head;
        for _ in 0..idx {
            cur_link = &cur_link.as_ref().unwrap().next;
        }

        Python::with_gil(|py| {
            Ok(cur_link.as_ref().unwrap().elem.clone_ref(py))
        })
    }
}

/// Private methods
impl LinkedList {
    fn reverse_index(&self, idx: usize) -> PyResult<usize> {
        Ok(
            self.size - idx - 1
        )
    }
}