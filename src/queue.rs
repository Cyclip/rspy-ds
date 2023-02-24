//! A queue data structure with a fixed capacity

use pyo3::prelude::*;

#[pyclass]
/// A queue data structure with a fixed capacity
pub struct Queue {
    queue: Vec<PyObject>,
    capacity: usize,
}

#[pymethods]
impl Queue {
    #[new]
    fn new(capacity: usize) -> Self {
        Queue {
            queue: Vec::new(),
            capacity,
        }
    }

    /// Add an item to the queue
    fn enqueue(&mut self, item: PyObject) -> PyResult<()> {
        if self.queue.len() == self.capacity {
            return Err(PyErr::new::<pyo3::exceptions::PyIndexError, _>(
                "Queue is full",
            ));
        }
        self.queue.push(item);
        Ok(())
    }

    /// Remove an item from the queue
    fn dequeue(&mut self) -> PyResult<PyObject> {
        if self.queue.is_empty() {
            return Err(PyErr::new::<pyo3::exceptions::PyIndexError, _>(
                "Queue is empty",
            ));
        }
        Ok(self.queue.remove(0))
    }

    /// Get the length of the queue
    fn len(&self) -> usize {
        self.queue.len()
    }

    /// Get the capacity of the queue
    fn capacity(&self) -> usize {
        self.capacity
    }

    /// Check if the queue is empty
    fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    /// Check if the queue is full
    fn is_full(&self) -> bool {
        self.queue.len() == self.capacity
    }
}