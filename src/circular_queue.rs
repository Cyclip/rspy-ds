use pyo3::{prelude::*, exceptions::PyIndexError};

#[pyclass]
/// A circular queue data structure with a fixed capacity
pub struct CircularQueue {
    queue: Vec<Option<PyObject>>,
    max_index: usize,
    head: usize,
    tail: usize,
}

#[pymethods]
impl CircularQueue {
    #[new]
    fn new(capacity: usize) -> Self {
        CircularQueue {
            queue: std::iter::repeat_with(|| None).take(capacity).collect(),
            max_index: capacity - 1,
            head: 0,
            tail: 0,
        }
    }

    /// Add an item to the queue
    fn enqueue(&mut self, item: PyObject) -> PyResult<()> {
        if (
            self.tail == self.max_index && self.head == 1
        ) || (
            self.tail + 1 != 0
        ) {
            // set the item at the tail
            self.queue[self.tail] = Some(item);

            println!("[Enqueue] Queue: {:?}", self.queue);

            if self.tail == self.max_index {
                self.tail = 0;
            } else {
                self.tail += 1;
            }


            Ok(())
            
        }
        else {
            return Err(PyErr::new::<PyIndexError, _>("Queue is full"));
        }
    }

    /// Remove an item from the queue
    fn dequeue(&mut self) -> PyResult<PyObject> {
        let val = match self.queue[self.head] {
            Some(ref item) => item.clone(),
            None => return Err(PyErr::new::<PyIndexError, _>("Queue is empty")),
        };
        self.queue[self.head] = None;

        if self.head == self.max_index {
            self.head = 0;
        } else {
            self.head += 1;
        }

        Ok(val)
    }
}